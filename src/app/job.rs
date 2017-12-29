use super::prelude::*;
use repo::RepoInfo;
use std::time::SystemTime;
use chrono::prelude::*;
use models::schema;

pub fn search_by_language(conn: &Connect) {
    // get all language
    let languages = SearchLanguage::get_all_language(&conn.db);
    for l in languages {
        let url = format!(
            "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc",
            l.language
        );
        let mut page = Page2::new(&url, 0, 100, true);

        let url_list = page.get_url_list();

        for url in url_list {
            if let Ok(res) = page.get(&url) {
                match exec_result(conn, res) {
                    Ok(()) => info!("{:?} . succ", url),
                    Err(e) => warn!("{:?}", e),
                }
            }
        }
    }
}


pub fn exec_result(conn: &Connect, res: Vec<RepoInfo>) -> Result<(), Error> {
    let now = SystemTime::now();
    let insert_date = &Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    for one in &res {
        // save owner
        let owner = &one.owner;
        // exist owner, skip
        // todo if changed
        if owner::find_owner_by_id(&conn.db, owner.id).len() == 0 {
            let data = NewOwner::new(
                owner.id,
                &owner.login,
                &owner.avatar_url,
                &owner.gravatar_id,
                &owner.url,
                &owner.html_url,
                &owner.user_type,
                owner.site_admin,
            );
            conn.save(owners::table, &data);
        }
        // exist return id, or insert and return id
        let l = &one.license;
        let license_id = License::find_id(&conn.db, &l.key, &l.name, &l.spdx_id, &l.url);

        let base_info = &one.base_info;

        let create_at = one.created_at.parse::<DateTime<Local>>()?;
        let create_at = SystemTime::from(create_at);

        let base = repo_base::NewRepoBase {
            id: base_info.id,
            license_id,
            owner_id: owner.id,
            name: &base_info.repo_name,
            full_name: &base_info.get_full_name()?,
            private: one.private,
            html_url: &one.html_url,
            description: &one.description,
            create_at: create_at,
            homepage: &one.homepage,
            language: &base_info.language,
            insert_time: now,
        };
        conn.save(schema::repo_base::table, &base);

        let updated_at = one.updated_at.parse::<DateTime<Local>>()?;
        let updated_at = SystemTime::from(updated_at);
        let info = repo_info::NewRepoInfo {
            id: None,
            base_id: base_info.id,
            license_id,
            owner_id: owner.id,
            insert_date: insert_date,
            size: one.size,
            stars: one.stargazers_count as i32,
            forks: one.forks as i32,
            issues: one.open_issues as i32,
            language: &base_info.language,
            updated_at: updated_at,
            has_pages: one.has_pages,
            has_wiki: one.has_wiki,
            has_downloads: one.has_downloads,
            has_issues: one.has_issues,
            create_at: now,
        };
        conn.save(schema::repo_infos::table, &info);

        println!("succ. {}", license_id);
    }

    Ok(())
}
