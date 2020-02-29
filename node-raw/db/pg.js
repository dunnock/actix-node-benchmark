const { Pool } = require('pg')

const pool = new Pool({
  host: process.env.DB_HOST,
  user: 'sped',
  password : 'sped',
  database: 'sped',
  max: process.env.POOL_SIZE || '10',
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

async function get_tasks() {
	return {
		client: await pool.connect(),
		config: {
			name: "get_tasks",
			text: `SELECT tasks.id, tasks.summary, assignee.id, assignee.name 
				FROM tasks INNER JOIN workers as assignee ON assignee.id = tasks.assignee_id
				WHERE ($1::varchar is null or assignee.name LIKE $1) AND ($2::varchar is null or summary LIKE $2) LIMIT $3::int`,
			rowMode: "array"
		}
	};
}

async function get_tasks_full() {
	return {
		client: await pool.connect(),
		config: {
			name: "get_tasks",
			text: `SELECT tasks.id, tasks.summary, assignee.id, assignee.name, tasks.description
				FROM tasks INNER JOIN workers as assignee ON assignee.id = tasks.assignee_id
				WHERE ($1::varchar is null or assignee.name LIKE $1) AND ($2::varchar is null or summary LIKE $2) LIMIT $3::int`,
			rowMode: "array"
		}
	};
}

module.exports = { get_tasks, get_tasks_full };