SELECT 
	task.id as id, 
	task.summary as summary, 
	assignee.id as assignee_id, 
	assignee.name as assignee_name,
	null as description
FROM task INNER JOIN worker as assignee ON assignee.id = task.assignee_id
WHERE ($1 is null or assignee.name LIKE $1) AND ($2 is null or summary LIKE $2) LIMIT $3