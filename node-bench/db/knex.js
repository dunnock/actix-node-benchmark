const knex = require('knex')({
    client: 'pg',
    connection: {
        host : process.env.DB_HOST,
        user : 'sped',
        password : 'sped',
        database : 'sped'
    },
    pool: {
        min: 5,
        max: 15
    }
});

const task  = require('../model/task.js');

function tasks() { 
    return knex.from('task')
        .innerJoin('worker as assignee', 'assignee.id', 'task.assignee_id');
}

function query_get_tasks() {
    return tasks()
        .select('task.id', 'task.summary', 'assignee.id as assignee_id', 'assignee.name as assignee_name');
}

function query_get_tasks_full() {
    return tasks()
        .select('task.id', 'task.summary', 'task.description', 'assignee.id as assignee_id', 'assignee.name as assignee_name');
}

async function get_tasks(assignee_name, summary, limit, full) {
    let query = full ? query_get_tasks_full() : query_get_tasks();
    
    if (!!assignee_name) {
        query.where("assignee.name", "LIKE", `%${assignee_name}%`)
    }
    if (!!summary) {
        query.where("summary", "LIKE", `%${summary}%`)
    }
    query.limit(limit || 10);
    
    let rows = await query;

    return rows.map(row => task.fromRow(row));
}




module.exports = {
    get_tasks
};