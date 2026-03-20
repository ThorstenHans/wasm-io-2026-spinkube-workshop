use anyhow::Result;
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};

use spin_sdk::pg4::{Connection, Decode, ParameterValue};
use spin_sdk::{http_component, variables};

use crate::models::TaskStatus;

mod constants;
mod models;

fn bad_request() -> Result<Response> {
    Ok(Response::new(400, "Bad Request"))
}

#[http_component]
fn handle_http_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/tasks", get_all_tasks);
    router.post("/tasks", add_task);
    router.put("/tasks/:id", toggle_task);

    Ok(router.handle(req))
}

fn get_all_tasks(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let connection_string = variables::get(constants::VAR_NAME_CONNECTION_STRING)?;
    let con = Connection::open(&connection_string)?;
    let params: Vec<ParameterValue> = vec![];
    let result = con.query(constants::GET_ALL_TASKS, &params)?;
    let mut tasks = models::TaskList::new();
    for row in result.rows {
        let task = models::TaskModel {
            id: i32::decode(&row[0]).expect("Could not decode task id"),
            contents: String::decode(&row[1]).expect("Could not decode task contents"),
            status: String::decode(&row[1])
                .expect("Could not decode task status")
                .parse()
                .expect("Could turn string into TaskStatus"),
        };
        tasks.add_task(task);
    }
    Ok(ResponseBuilder::new(200)
        .header("Content-Type", "application/json")
        .body(tasks)
        .build())
}

fn add_task(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<models::AddTaskModel>(&req.body()) else {
        return bad_request();
    };
    let connection_string = variables::get(constants::VAR_NAME_CONNECTION_STRING)?;

    let con = Connection::open(&connection_string)?;
    let params = vec![
        ParameterValue::Str(model.contents),
        ParameterValue::Str(String::from(TaskStatus::Open)),
    ];
    con.execute(constants::ADD_TASK, &params)?;

    Ok(Response::new(201, ()))
}

fn toggle_task(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return bad_request();
    };

    let Ok(id) = id.parse::<i32>() else {
        return bad_request();
    };

    let p: Vec<ParameterValue> = vec![ParameterValue::Int32(id)];
    let connection_string = variables::get(constants::VAR_NAME_CONNECTION_STRING)?;
    let con = Connection::open(&connection_string)?;
    con.execute(constants::TOGGLE_TASK, &p)?;

    Ok(Response::new(204, ()))
}
