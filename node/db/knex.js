module.exports = require('knex')({
	client: 'pg',
	connection: {
		host : process.env.DB_HOST,
		user : 'db_user',
		password : 'admin',
		database : 'db'
	},
	pool: {
		min: 2,
		max: 10
	}
});