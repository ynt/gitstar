CREATE TABLE owners (
	id int8 PRIMARY KEY,
	login text,
	avatar_url text,
	gravatar_id text,
	url text,
	html_url text,
	user_type text,
	site_admin bool,
	publish_at timestamp,
	update_at timestamp
);