const knex = require('../db/knex.js');

const tasks = knex.from('tasks')
	.innerJoin('workers as assignee', 'assignee.id', 'tasks.assignee_id');

function get_task(id) {
	return tasks.where('id', id).select('tasks.id', 'tasks.summary', 'tasks.description', 'assignee.id', 'assignee.name');
}

function get_tasks(query) {
	return tasks.where(query).select('tasks.id', 'tasks.summary', 'tasks.description', 'assignee.id', 'assignee.name');
}

module.exports = {
	get_task,
	get_tasks
};
