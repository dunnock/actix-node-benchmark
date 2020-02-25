const knex = require('../db/knex.js');

function tasks() { 
    return knex.from('tasks')
        .innerJoin('workers as assignee', 'assignee.id', 'tasks.assignee_id');
}

function get_task(id) {
    return tasks().select('tasks.id', 'tasks.summary', 'tasks.description', 'assignee.id as assignee_id', 'assignee.name').where('id', id);
}

function get_tasks() {
    return tasks().select('tasks.id', 'tasks.summary', 'assignee.id', 'assignee.name');
}

function get_tasks_with_description() {
    return tasks().select('tasks.id', 'tasks.summary', 'tasks.description', 'assignee.id', 'assignee.name');
}

module.exports = {
    get_task,
    get_tasks
};
