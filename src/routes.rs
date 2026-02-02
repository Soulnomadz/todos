use salvo::prelude::*;
use crate::handler::*;
use salvo::basic_auth::BasicAuth;
use salvo::session::{CookieStore, Session, SessionDepotExt, SessionHandler};

pub fn route() -> Router {
    //let auth_handler = BasicAuth::new(crate::middleware::Validator);

    let session_handler = SessionHandler::builder(
	CookieStore::new(),
	b"secretabsecretabsecretabsecretabsecretabsecretabsecretabsecretab",
    )
    .cookie_name("xxx-cn")
    .build()
    .unwrap();

    Router::new()
	//.hoop(auth_handler)
	.hoop(session_handler)
        .push(Router::new().get(index))
	.push(Router::new().path("login").post(login))
        .push(Router::new().path("logout").get(logout))
        .push(Router::new().path("todos").push(todo_route()))
	.push(
	    Router::with_path("/static/{**path}")
		.get(StaticDir::new(["static"]))
	)
}

fn todo_route() -> Router {
    Router::new()
        .get(list_todos)
        .post(create_todo)
        .push(
            Router::with_path("{id}")
		.get(get_todo_by_id)
                .put(update_todo)
                .delete(delete_todo)
        )
}

