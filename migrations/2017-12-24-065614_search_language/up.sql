CREATE TABLE search_languages (
	id SERIAL PRIMARY KEY,
	language text,
	status bool,
	publish_at timestamp,
	update_at timestamp
);