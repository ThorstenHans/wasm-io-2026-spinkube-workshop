use std::str::FromStr;

use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Open,
    Done,
    Archived,
}

impl From<TaskStatus> for String {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::Open => String::from("open"),
            TaskStatus::Done => String::from("done"),
            TaskStatus::Archived => String::from("archived"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(TaskStatus::Open),
            "done" => Ok(TaskStatus::Done),
            "archived" => Ok(TaskStatus::Archived),
            _ => Err(format!("'{}' is not a valid TaskStatus", s)),
        }
    }
}

#[derive(Deserialize)]
pub struct AddTaskModel {
    pub contents: String,
}

#[derive(Serialize)]
pub struct TaskList {
    tasks: Vec<TaskModel>,
}

#[derive(Serialize)]
pub(crate) struct TaskModel {
    pub id: i32,
    pub contents: String,
    pub status: TaskStatus,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: vec![] }
    }

    pub fn add_task(&mut self, value: TaskModel) {
        self.tasks.push(value);
    }
}

impl IntoBody for TaskList {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self.tasks).expect("could not serialize task into JSON payload")
    }
}
