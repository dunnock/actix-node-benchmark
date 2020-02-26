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

    return rows.map((row) => ({
        id: row['id'],
        summary: row['summary'], 
        assignee_id: row['assignee_id'], 
        assignee_name: row['assignee_name'],
        description: full ? row['description'] : null
    }));
}




module.exports = {
    get_task,
    get_tasks
};