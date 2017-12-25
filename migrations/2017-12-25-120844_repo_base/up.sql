CREATE TABLE repo_base (
	id int8 PRIMARY KEY,
	license_id int8,
	owner_id int8,
	name text,
	full_name text,
	private bool,
	html_url text,
	description text,
	create_at timestamp,
	homepage text,
	lanauage text,
	insert_time timestamp
);