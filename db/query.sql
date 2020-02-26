SELECT task.id, task.summary, task.description, assignee.id, assignee.name FROM task INNER JOIN worker as assignee ON assignee.id = task.assignee_id

DELETE FROM worker;
DELETE FROM task;

SELECT * FROM worker;