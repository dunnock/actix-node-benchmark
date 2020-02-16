const express = require('express')
const router = express.Router()
const { get_task, get_tasks } = require('../rest/api.js')


router.get('/tasks/:id', function(req, res) {
	const values = []
	const id = req.params.id;

	get_task(id, res);
})

router.get('/tasks', (req, res) => {
	let query = function (builder) {
		if (!!req.query["assignee_name"]) {
			builder.where("assignee.name", "LIKE", req.query["assignee_name"])
		}
		if (!!req.query["summary"]) {
			builder.where("summary", "LIKE", req.query["summary"])
		}
	};
	get_tasks(query, req.query.offset || 0, res);
})

module.exports = router