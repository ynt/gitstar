table!{
    owners {
        id -> BigInt,
        login -> Text,
        avatar_url -> Text,
        gravatar_id -> Text,
        url -> Text,
        html_url -> Text,
        user_type -> Text,
        site_admin -> Bool,
        publish_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table!{
    search_languages {
        id -> Serial,
        language -> Text,
        status -> Bool,
        publish_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    license {
        id -> Serial,
        key -> Text,
        name -> Text,
        spdx_id -> Text,
        url -> Text,
    }
}

table! {
    repo_base {
        id -> BigInt,
        license_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        full_name -> Text,
        private -> Bool,
        html_url -> Text,
        description -> Text,
        create_at -> Timestamp,
        homepage -> Text,
        lanauage -> Text,
        insert_time -> Timestamp,
    }
}

table! {
    repo_info {
        id -> Serial,
        base_id -> BigInt,
        license_id -> BigInt,
        owner_id -> BigInt,
        insert_date -> Timestamp,
        size -> BigInt,
        stars -> Integer,
        forks -> Integer,
        issues -> Integer,
        language -> Text,
        updated_at -> Timestamp,
        has_pages -> Bool,
        has_wiki -> Bool,
        has_downloads -> Bool,
        has_issues -> Bool,
        create_at -> Timestamp,
    }
}
