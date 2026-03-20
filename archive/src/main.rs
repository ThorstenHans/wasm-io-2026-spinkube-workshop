use anyhow::{Context, Result};
use chrono::Utc;
use spin_sdk::{
    pg4::{Connection, Decode, ParameterValue},
    variables,
};
use std::process;
mod constants;

fn main() {
    match archive_completed_tasks() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error while archiving completed tasks: {}", e);
            process::exit(1)
        }
    }
}

fn archive_completed_tasks() -> Result<()> {
    let connection_string = variables::get(constants::VAR_NAME_CONNECTION_STRING)
        .with_context(|| "Could not get connection string")?;

    let con = Connection::open(&connection_string)
        .with_context(|| "Could not establish connection to PostgreSQL")?;

    let params: Vec<ParameterValue> = vec![];
    let completed_tasks = con
        .query(constants::GET_COMPLETED_TASKS, &params)
        .with_context(|| "Could not retrieve completed tasks")?;
    if completed_tasks.rows.is_empty() {
        println!("No completed tasks found. Will terminate now...");
        return Ok(());
    }
    println!("Iterating over all completed tasks now");
    for row in completed_tasks.rows {
        // archive task adn log it
        let task_id = i32::decode(&row[0]).with_context(|| "Could not decode Task identifier")?;
        println!("Task with id {} is completed. Will archive it now", task_id);
        let now = format!("{}", Utc::now().format("%y-%d-%m"));
        let archive_params = vec![ParameterValue::Int32(task_id)];
        con.execute(constants::ARCHIVE_TASK, &archive_params)
            .map_err(|e| anyhow::anyhow!("Could not archive task {}: {}", task_id, e))?;

        let msg = format!("Completed Task ({}) has been archived by cron", task_id);

        let audit_params = vec![
            ParameterValue::Int32(task_id),
            ParameterValue::Str(now),
            ParameterValue::Str(msg),
        ];
        con.execute(constants::AUDIT_ARCHIVING, &audit_params)
            .with_context(|| format!("Could not write audit when archiving task {}", task_id))?;
    }
    println!("Done with processing tasks");
    Ok(())
}
