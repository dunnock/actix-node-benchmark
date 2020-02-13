const knex = require('../db/knex.js');

const tasks = knex.from('tasks')
	.innerJoin('workers as assignee', 'assignee.id', 'tasks.assignee_id');

function get_task(id) {
	return tasks.where('id', id).select('tasks.summary', 'tasks.created', 'assignee.name');
}

function get_tasks(query) {
	return tasks.where(query).select('tasks.summary', 'tasks.created', 'assignee.name');
}

module.exports = {
	get_task,
	get_tasks
};
