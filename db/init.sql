CREATE TABLE workers (
	id integer PRIMARY KEY,
	name varchar(255) NOT NULL,
	email varchar(255) NULL,
	score integer DEFAULT 0
)

CREATE TABLE tasks (
	id integer PRIMARY KEY,
	summary varchar(255) NOT NULL,
	description text NOT NULL,
	assignee integer NULL REFERENCES workers
)

CREATE INDEX IF NOT EXISTS tasks_search ON TASKS (
	assignee, summary
)

CREATE INDEX IF NOT EXISTS workers_search_name ON TASKS (
	name
)

CREATE INDEX IF NOT EXISTS workers_search_email ON TASKS (
	email
)