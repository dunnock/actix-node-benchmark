const db = require('../db/pg.js');

async function get_tasks(params, offset, res) {
    let conn = (params["full"] == "true")
        ? db.get_tasks_full()
        : db.get_tasks();

    let { client, config } = await conn;

    let values = [
        !!params["assignee_name"] ? `%${params["assignee_name"]}%` : null,
        !!params["summary"] ? `%${params["summary"]}%` : null,
        params["limit"] || 10
    ];
    let query = Object.assign(config, { values });

    let resp = await client.query(query);

    let result = resp.rows.map((row) => ({
        id: row[0],
        summary: row[1], 
        assignee_id: row[2], 
        assignee_name: row[3],
        description: (params["full"] == "true") ? row[4] : null
    }));

    res.send(result);

    client.release();
}

module.exports = {
    get_tasks
};
