'use strict'

function fromRow(row) {
	return {
		id: row['id'],
		summary: row['summary'], 
		assignee_id: row['assignee_id'], 
		assignee_name: row['assignee_name'],
		description: row['description'] || null
	}
}

module.exports = {fromRow};