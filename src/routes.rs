use salvo::prelude::*;
use crate::handler::*;

pub fn route() -> Router {
    Router::new()
        .push(Router::new().get(index))
        .push(Router::new().path("todos").push(todo_route()))
}

fn todo_route() -> Router {
    Router::new()
        .get(list_todos)
        .post(create_todo)
        .push(
            Router::with_path("{id}")
                .put(update_todo)
                .delete(delete_todo)
        )
}
