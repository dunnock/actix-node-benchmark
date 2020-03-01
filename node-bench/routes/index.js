'use strict'

const express = require('express')
const router = express.Router()
const db = require('../db/knex.js');
const { createError } = require('./error.js');

router.get('/tasks', (req, res, next) => {
    let { assignee_name, summary, limit, full } = req.query;
    full = full == "true";
    if (!!limit && isNaN(limit)) {
        return next(createError({
            status: 400, 
            message: "limit query parameter should be a number"
        }));
    }

    db.get_tasks(assignee_name, summary, limit, full)
        .then(tasks => res.send(tasks))
        .catch(err => next(createError(err)))
})

module.exports = router