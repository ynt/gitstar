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
