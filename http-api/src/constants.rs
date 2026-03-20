// have fewer magic strings
pub(crate) const VAR_NAME_CONNECTION_STRING: &str = "pg_connection_string";

// sql commands executed against the database
pub(crate) const ADD_TASK: &str = "INSERT INTO tasks (contents, status) VALUES ($1,$2)";
pub(crate) const GET_ALL_TASKS: &str = "SELECT id, contents, status FROM tasks";
pub(crate) const TOGGLE_TASK: &str = "UPDATE tasks SET status = CASE WHEN status = 'open' THEN 'done' WHEN status = 'done' THEN 'open' ELSE status END WHERE id = $1";

