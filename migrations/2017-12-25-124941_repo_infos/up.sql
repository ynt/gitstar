CREATE TABLE repo_infos (
	id SERIAL PRIMARY KEY,
	base_id int8,
	license_id int8,
	owner_id int8,
	insert_date text,
	size int8,
	stars int4,
	forks int4,
	issues int4,
	language text,
	updated_at timestamp,
	has_pages bool,
	has_wiki bool,
	has_downloads bool,
	has_issues bool,
	create_at timestamp
);