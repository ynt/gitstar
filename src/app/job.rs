use super::repo::RepoInfo;
use super::prelude::*;

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
                exec_result(conn, res);
            }
        }
    }

}

pub fn exec_result(conn: &Connect, res: Vec<RepoInfo>) {
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
            println!("succ");
        }
    }
}
