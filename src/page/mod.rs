use std::str;
use serde_json::Value;
use url::Url;
use regex::Regex;
use std::str::FromStr;

use super::client;
use super::repo::*;
use super::util::*;
use super::error::Error;

#[derive(Debug, Default)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
    pub url: String,
    pub get_items: bool,

    scrapy_once: bool,
    first_get: bool,
    current_page_size: i64,
}

impl Page {
    pub fn new(url: &str, page_size: i64) -> Self {
        Page {
            url: url.to_owned(),
            page_size: page_size,

            ..Default::default()
        }
    }

    pub fn fetch(&mut self) -> Result<Vec<RepoInfo>, Error> {
        if self.is_end() {
            Err(Error::new("End of page.".to_owned()))
        } else {
            Ok(self.get()?)
        }
    }

    pub fn is_end(&mut self) -> bool {
        if self.first_get && self.current_page_size < self.page_size {
            true
        } else {
            false
        }
    }

    fn get(&mut self) -> Result<Vec<RepoInfo>, Error> {
        // if set page_size is zero, return a error.
        if self.page_size <= 0 {
            return Err(Error::new("Page's page_size lte zero.".to_owned()));
        }

        self.first_get = true;

        // if have next page, set page number.
        let mut url = self.url.clone();
        if self.page > 1 {
            if self.get_items {
                url.push_str("&page=");
                url.push_str(&self.page.to_string());
            } else {
                url.push_str("?page=");
                url.push_str(&self.page.to_string());
            }
            url.push_str("&per_page=100");
        } else {
            self.page = 1;
            url.push_str("?per_page=100");
        }
        self.page = self.page + 1;

        // Send a get request, get serde_json::Value object/array
        let res = client::get(&url)?;
        let result = res.to_json()?;

        let header = res.header;
        if let Some(raw_link) = header.get_raw("Link") {
            let links = str::from_utf8(raw_link.one().unwrap()).unwrap();

            let link: Vec<&str> = links.split(",").collect();

            println!("{:?}", link);
        }

        let mut user_repo: Vec<RepoInfo> = Default::default();

        // if result is a array
        if result.is_array() {
            for res in result.as_array().unwrap() {
                user_repo.push(create_repo_info(res));
            }
        } else if result.is_object() {
            if self.get_items {
                for res in result["items"].as_array().unwrap() {
                    user_repo.push(create_repo_info(res));
                }
            } else {
                user_repo.push(create_repo_info(&result));
            }
        }

        // Set current page size
        self.current_page_size = user_repo.len() as i64;

        // Return a Vec<RepoInfo>
        Ok(user_repo)
    }
}

#[derive(Debug, Default)]
pub struct Page2<'a> {
    url: &'a str,
    page_size: u64,
    page: u64,
    get_items: bool,
    first_fetch: bool,
    max_page: u64,
    is_end: bool,
}

impl<'a> Page2<'a> {
    pub fn new(url: &'a str, mut page: u64, page_size: u64, get_items: bool) -> Self {
        if page == 0 {
            page = 1;
        }

        Page2 {
            url: url,
            page: page,
            page_size: page_size,
            first_fetch: true,
            get_items: get_items,

            ..Default::default()
        }
    }

    pub fn get(&mut self, url: &str) -> Result<Vec<RepoInfo>, Error> {
        Ok(self.analyze_result(url)?)
    }

    pub fn get_url_list(&mut self) -> Vec<String> {
        if self.first_fetch {
            let url = self.get_next_url();
            let _ = self.analyze_result(&url);
        }

        let url = Url::parse(self.url).unwrap();
        let mut list: Vec<String> = Default::default();
        if self.max_page <= 1 {
            list.push(self.url.to_owned());
        } else {
            for pat in 1..self.max_page + 1 {
                let mut u = url.clone();
                u.query_pairs_mut().append_pair("page", &pat.to_string());
                u.query_pairs_mut().append_pair(
                    "per_page",
                    &self.page_size.to_string(),
                );
                list.push(u.as_str().to_string())
            }
        }

        list
    }

    pub fn get_next_url(&mut self) -> String {
        if self.first_fetch {
            // self.url.to_owned()
            self.splice_url()
        } else {
            if self.page_size != 0 {
                if self.page >= self.max_page {
                    self.is_end = true;
                    self.url.to_owned()
                } else {
                    self.page += 1;
                    self.splice_url()
                }
            } else {
                self.is_end = true;
                self.url.to_owned()
            }
        }
    }

    pub fn is_end(&self) -> bool {
        self.is_end
    }

    pub fn fetch(&mut self) -> Result<Vec<RepoInfo>, Error> {
        let url = self.get_next_url();

        Ok(self.analyze_result(&url)?)
    }

    fn splice_url(&mut self) -> String {
        let mut url = Url::parse(self.url).unwrap();

        if self.page > 1 {
            url.query_pairs_mut().append_pair(
                "page",
                &self.page.to_string(),
            );
        }

        if self.page_size != 0 {
            url.query_pairs_mut().append_pair(
                "per_page",
                &self.page_size.to_string(),
            );
        }

        url.into_string()
    }

    fn analyze_result(&mut self, url: &str) -> Result<Vec<RepoInfo>, Error> {
        info!("fetch url: {}", url);

        // Send a get request, get serde_json::Value object/array
        let res = client::get(&url)?;
        let result = res.to_json()?;
        if self.first_fetch {
            let header = &res.header;
            if let Some(raw_link) = header.get_raw("Link") {
                let links = str::from_utf8(raw_link.one().expect("1")).expect("2");

                let re = Regex::new(r#"page=(?P<max_page>\d+)>; rel="last""#).expect("3");

                let caps = re.captures(links).unwrap();

                self.max_page = FromStr::from_str(&caps["max_page"]).unwrap();
            } else {
                self.max_page = 1;
            }

            if self.page >= self.max_page {
                self.is_end = true;
            }

            self.first_fetch = false;
        }
        let mut user_repo: Vec<RepoInfo> = Default::default();

        // if result is a array
        if result.is_array() {
            for res in result.as_array().unwrap() {
                user_repo.push(create_repo_info(res));
            }
        } else if result.is_object() {
            if self.get_items {
                for res in result["items"].as_array().unwrap() {
                    user_repo.push(create_repo_info(res));
                }
            } else {
                user_repo.push(create_repo_info(&result));
            }
        }
        Ok(user_repo)
    }
}

// Create RepoInfo by serde_json::Value
fn create_repo_info(res: &Value) -> RepoInfo {
    let base_info = BaseInfo {
        id: get_option_i64(res["id"].as_i64()),
        repo_name: get_option_string(res["name"].as_str()),
        username: get_option_string(res["owner"]["login"].as_str()),
        language: get_option_string(res["language"].as_str()),
    };

    let owner = Owner {
        login: base_info.username.clone(),
        id: get_option_i64(res["owner"]["id"].as_i64()),
        avatar_url: get_option_string(res["owner"]["avatar_url"].as_str()),
        gravatar_id: get_option_string(res["owner"]["gravatar_id"].as_str()),
        url: get_option_string(res["owner"]["url"].as_str()),
        html_url: get_option_string(res["owner"]["html_url"].as_str()),
        followers_url: get_option_string(res["owner"]["followers_url"].as_str()),
        following_url: get_option_string(res["owner"]["following_url"].as_str()),
        gists_url: get_option_string(res["owner"]["gists_url"].as_str()),
        starred_url: get_option_string(res["owner"]["starred_url"].as_str()),
        subscriptions_url: get_option_string(res["owner"]["subscriptions_url"].as_str()),
        organizations_url: get_option_string(res["owner"]["organizations_url"].as_str()),
        repos_url: get_option_string(res["owner"]["repos_url"].as_str()),
        events_url: get_option_string(res["owner"]["events_url"].as_str()),
        received_events_url: get_option_string(res["owner"]["received_events_url"].as_str()),
        user_type: get_option_string(res["owner"]["user_type"].as_str()),
        site_admin: get_option_bool(res["owner"]["site_admin"].as_bool()),
    };

    let license = License {
        key: get_option_string(res["license"]["key"].as_str()),
        name: get_option_string(res["license"]["name"].as_str()),
        spdx_id: get_option_string(res["license"]["spdx_id"].as_str()),
        url: get_option_string(res["license"]["url"].as_str()),
    };

    RepoInfo {
        base_info: base_info,
        owner: owner,
        license: license,

        private: get_option_bool(res["private"].as_bool()),
        html_url: get_option_string(res["html_url"].as_str()),
        description: get_option_string(res["description"].as_str()),
        fork: get_option_bool(res["fork"].as_bool()),
        created_at: get_option_string(res["created_at"].as_str()),
        updated_at: get_option_string(res["updated_at"].as_str()),
        pushed_at: get_option_string(res["pushed_at"].as_str()),
        git_url: get_option_string(res["git_url"].as_str()),
        ssh_url: get_option_string(res["ssh_url"].as_str()),
        clone_url: get_option_string(res["clone_url"].as_str()),
        svn_url: get_option_string(res["svn_url"].as_str()),
        homepage: get_option_string(res["homepage"].as_str()),
        size: get_option_i64(res["size"].as_i64()),
        stargazers_count: get_option_i64(res["stargazers_count"].as_i64()),
        watchers_count: get_option_i64(res["watchers_count"].as_i64()),
        language: get_option_string(res["language"].as_str()),
        has_issues: get_option_bool(res["has_issues"].as_bool()),
        has_projects: get_option_bool(res["has_projects"].as_bool()),
        has_downloads: get_option_bool(res["has_downloads"].as_bool()),
        has_wiki: get_option_bool(res["has_wiki"].as_bool()),
        has_pages: get_option_bool(res["has_pages"].as_bool()),
        forks_count: get_option_i64(res["forks_count"].as_i64()),
        mirror_url: get_option_string(res["mirror_url"].as_str()),
        archived: get_option_bool(res["archived"].as_bool()),
        open_issues_count: get_option_i64(res["open_issues_count"].as_i64()),

        forks: get_option_i64(res["forks"].as_i64()),
        open_issues: get_option_i64(res["open_issues"].as_i64()),
        watchers: get_option_i64(res["watchers"].as_i64()),
        default_branch: get_option_string(res["default_branch"].as_str()),
        ..Default::default()
    }
}
