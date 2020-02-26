const express = require('express')
const router = express.Router()
const pg = require('../rest/api_pg.js')
const db = require('../db/api.js');

router.get('/tasks', (req, res) => {
    let { assignee_name, summary, limit, full } = req.query;
    full = full == "true"; 

    db.get_tasks(assignee_name, summary, limit, full)
        .then(tasks => res.send(tasks))
        .catch(err => {throw err})
})

router.get('/tasks_pg', (req, res) => {
    pg.get_tasks(req.query, res);
})

module.exports = router