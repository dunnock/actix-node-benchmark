'use strict'

const express = require('express')
const router = express.Router()
const db = require('../db/api.js');

router.get('/tasks', (req, res) => {
    let { assignee_name, summary, limit, full } = req.query;
    full = full == "true"; 

    db.get_tasks(assignee_name, summary, limit, full)
        .then(tasks => res.send(tasks))
        .catch(createError)
})

module.exports = router