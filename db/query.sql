SELECT tasks.id, tasks.summary, tasks.description, assignee.id, assignee.name FROM tasks INNER JOIN workers as assignee ON assignee.id = tasks.assignee_id

DELETE FROM workers;
DELETE FROM tasks;

SELECT * FROM workers;