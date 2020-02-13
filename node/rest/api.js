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

function get_tasks(query, offset, res) {

	db.get_tasks(query).offset(offset).limit(20).then(rows => {
		res.send(rows)
	})
		.catch(err => {throw err})

	res.send(db.get_tasks(query));
}

module.exports = {
	get_task,
	get_tasks
};
