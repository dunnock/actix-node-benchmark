const express = require('express')
const router = express.Router()
const { get_task, get_tasks } = require('../rest/api.js')


router.get('/tasks/:id', function(req, res) {
	const values = []
	const id = req.params.id;

	get_task(id, res);
})

router.get('/tasks', (req, res) => {
	get_tasks(req.query, req.query.offset || 0, res);
})

module.exports = router