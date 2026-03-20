pub const GET_COMPLETED_TASKS: &str = "SELECT id from tasks where status = 'done' ";
pub const ARCHIVE_TASK: &str = "UPDATE tasks set status = 'archived' WHERE id = $1";
pub const AUDIT_ARCHIVING: &str = "INSERT into audit (task_id, ts, message) VALUES ($1, $2, $3)";
