const knex = require('knex')({
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

function tasks() { 
    return knex.from('task')
        .innerJoin('worker as assignee', 'assignee.id', 'task.assignee_id');
}

function get_task(id) {
    return tasks().select('task.id', 'task.summary', 'task.description', 'assignee.id as assignee_id', 'assignee.name').where('id', id);
}

function get_tasks() {
    return tasks().select('task.id', 'task.summary', 'assignee.id as assignee_id', 'assignee.name as assignee_name');
}

function get_tasks_full() {
    return tasks().select('task.id', 'task.summary', 'task.description', 'assignee.id as assignee_id', 'assignee.name as assignee_name');
}

module.exports = {
    get_task,
    get_tasks,
    get_tasks_full
};