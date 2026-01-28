use salvo::prelude::*;
use crate::db::get_pgpool;
use crate::types::*;

#[handler]
pub fn index(res: &mut Response) {
    res.render(Text::Html(
        "<a href=\"/todos\">going to the todo page</a>"
    ));     
}

#[handler]
pub async fn list_todos(req: &mut Request, res: &mut Response) {
    let todos: Vec<Todo> = sqlx::query_as!(
        Todo,
        "select id,text,completed from public.todos",
    )
        .fetch_all(get_pgpool())
        .await
        .map_err(|e| {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::BAD_REQUEST
        }).unwrap();

    res.render(Json(todos));
} 

#[handler]
pub async fn create_todo(req: &mut Request, res: &mut Response) {
    let new_todo: Todo = req
        .parse_body_with_max_size(512)
        .await
        .map_err(|e| {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::BAD_REQUEST
        }).unwrap();
    
    sqlx::query!(
        "insert into public.todos (id, text, completed) values ($1, $2, $3)",
        new_todo.id,
        new_todo.text,
        new_todo.completed,
    )
        .execute(get_pgpool())
        .await
        .map_err(|e| {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::BAD_REQUEST
        }).unwrap();

    tracing::debug!(todo = ?new_todo, "create todo");
}

#[handler]
pub async fn update_todo(req: &mut Request, res: &mut Response) {
    let raw_id = req.param("id").unwrap_or("id missin");

    let id = match req.try_param::<i64>("id") {
        Ok(id) => id,
        Err(e) => {
            tracing::debug!(
                error = %e,
                id = raw_id,
                "Error:"
            );

            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };

//    let raw_id: String = req.param("id")
//      .ok_or_else(|| {
//          tracing::debug!("id missin");
//          StatusError::bad_request().detail("id missing");
//          return;
//      }).unwrap();
//
//    let id = raw_id.parse::<i64>()
//      .map_err(|_| {
//          tracing::debug!(id = raw_id, "params: ");
//          StatusError::bad_request().detail("invalid id");
//          return;
//      }).unwrap();

    tracing::debug!(id = id, "params:");

    let todo: Todo = req
        .parse_body_with_max_size(512)
        .await
        .map_err(|e| {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::BAD_REQUEST
        }).unwrap();

    let result = sqlx::query!(
        "update public.todos set text = $1, completed = $2 where id = $3",
        todo.text,
        todo.completed,
        id,
    )
        .execute(get_pgpool())
        .await
        .unwrap();

    if result.rows_affected() == 0 {
        tracing::debug!("Error: id not match！");
        res.status_code(StatusCode::BAD_REQUEST);
    } else {
        tracing::debug!(todo = ?todo, "updated: ");
    }
}


#[handler]
pub async fn delete_todo(req: &mut Request, res: &mut Response) {
    let raw_id = req.param("id").unwrap_or("id missin");

    let id = match req.try_param::<i64>("id") {
        Ok(id) => id,
        Err(e) => {
            tracing::debug!(
                error = %e,
                id = raw_id,
                "Error:"
            );

            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };

    let result = sqlx::query!(
        "delete from public.todos where id = $1",
        id,
    )
        .execute(get_pgpool())
        .await
        .unwrap();

    if result.rows_affected() == 0 {
        tracing::debug!("Error: id not found！");
        res.status_code(StatusCode::BAD_REQUEST);
    } else {
        tracing::debug!(id = id, "deleted: ");
    }
}
