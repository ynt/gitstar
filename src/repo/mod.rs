use super::error::Error;
use super::page::Page;

#[derive(Debug)]
#[derive(Default)]
pub struct BaseInfo {
    pub id: i64,
    pub repo_name: String,
    pub username: String,
    pub language: String,
}

impl BaseInfo {
    pub fn new(id: i64, repo_name: String, username: String, language: String) -> Self {
        BaseInfo {
            id: id,
            repo_name: repo_name.to_lowercase(),
            username: username.to_lowercase(),
            language: language.to_lowercase(),
            ..Default::default()
        }
    }

    // https://api.github.com/users/gowins/repos
    pub fn get_user_repos(&self) -> Result<Page, Error> {
        if self.repo_name.is_empty() {
            return Err(Error::new("BaseInfo feild repo_name is empty".to_owned()));
        }

        let mut url = String::from("https://api.github.com/users/");

        url.push_str(&self.repo_name);
        url.push_str("/repos");

        Ok(Page::new(&url, 100))
    }

    // https://api.github.com/repos/google/gops
    pub fn get_repo_info(&self) -> Result<Page, Error> {
        let mut url = String::from("https://api.github.com/repos/");

        url.push_str(&self.get_full_name()?);

        Ok(Page::new(&url, 100))
    }

    // https://api.github.com/search/repositories?q=language:go&sort=stars&order=desc
    pub fn get_repo_by_search(&self) -> Result<Page, Error> {
        if self.language.is_empty() {
            return Err(Error::new("BaseInfo feild language is empty".to_owned()));
        }

        let mut url = String::from("https://api.github.com/search/repositories?q=language:");

        url.push_str(&self.language);

        url.push_str("&sort=stars&order=desc");

        let mut page = Page::new(&url, 100);
        page.get_items = true;

        Ok(page)
    }

    pub fn get_full_name(&self) -> Result<String, Error> {
        if self.repo_name.is_empty() {
            Err(Error::new("BaseInfo feild repo_name is empty".to_owned()))
        } else if self.username.is_empty() {
            Err(Error::new("BaseInfo feild username is empty".to_owned()))
        } else {
            Ok(self.repo_name.clone() + "/" + &self.username)
        }
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct RepoInfo {
    pub base_info: BaseInfo,
    pub owner: Owner,

    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    pub homepage: String,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: String,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: i64,
    pub mirror_url: String,
    pub archived: bool,
    pub open_issues_count: i64,

    pub license: License,

    pub forks: i64,
    pub open_issues: i64,
    pub watchers: i64,
    pub default_branch: String,
}

#[derive(Debug)]
#[derive(Default)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
}

#[derive(Debug)]
#[derive(Default)]
pub struct Owner {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub user_type: String,
    pub site_admin: bool,
}
