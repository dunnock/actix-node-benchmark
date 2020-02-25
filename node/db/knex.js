module.exports = require('knex')({
    client: 'pg',
    connection: {
        host : process.env.DB_HOST,
        user : 'sped',
        password : 'sped',
        database : 'sped'
    },
    pool: {
        min: 2,
        max: 10
    }
});