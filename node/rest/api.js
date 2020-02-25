const db = require('../db/api.js');

async function get_task(id, res) {
    db.get_task(id)
        .then((rows) => {
            if (rows.length == 1) {
                res.send({
                    id: row[0]['tasks.id'],
                    summary: row[0]['tasks.summary'], 
                    assignee_id: row[0]['assignee.id'], 
                    assignee_name: row[0]['assignee.name'],
                    description: row[0]['tasks.description']
                });
            } else {
                res.status(404).send("Task not found");
            }
        })
        .catch(err => {throw err})
}

function get_tasks(params, offset, res) {
    let query = (params["full"] == "true")
        ? db.get_tasks_with_description()
        : db.get_tasks();

    if (!!params["assignee_name"]) {
        query.where("assignee.name", "LIKE", "%" + params["assignee_name"] + "%")
    }
    if (!!params["summary"]) {
        query.where("summary", "LIKE", "%" + params["summary"] + "%")
    }
    query.limit(params["limit"] || 10);

    query
        .then(rows => {
        res.send(rows.map((row) => ({
            id: row['id'],
            summary: row['summary'], 
            assignee_id: row['assignee_id'], 
            assignee_name: row['assignee_name'],
            description: (params["full"] == "true") ? row['description'] : null
        })))
    })
    .catch(err => {throw err})
}

module.exports = {
    get_task,
    get_tasks
};
