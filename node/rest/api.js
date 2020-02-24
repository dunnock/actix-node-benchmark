const db = require('../db/api.js');
const redis = require('redis')
const cache = redis.createClient(6379, process.env.REDIS_HOST)

async function get_task(id, res) {
	cache.hget(`tasks:${id}`, function (err, task) {
		if (err) {
			console.log('Cache::get_task error ' + err)
			res.send(db.get_task(id));
		} else {
			if (task) {
				res.send(task);
			} else {
				task = db.get_task(id);
				if (task) {
					cache.set(id, task);
					res.send(task);
				} else {
					res.status(404).send("Task not found");
				}
			}
		}
	})
}

function get_tasks(params, offset, res) {
	let query = db.get_tasks();

	if (!!params["assignee_name"]) {
		query.where("assignee.name", "LIKE", "%" + params["assignee_name"] + "%")
	}
	if (!!params["summary"]) {
		query.where("summary", "LIKE", "%" + params["summary"] + "%")
	}
	query.limit(params["limit"] || 10);

	query.then(rows => {
		res.send(rows.map((row) => ({
			id: row['tasks.id'],
			summary: row['tasks.summary'], 
			assignee_id: row['assignee.id'], 
			assignee_name: row['assignee.name'],
			description: null
		})))
	})
	.catch(err => {throw err})
}

module.exports = {
	get_task,
	get_tasks
};
