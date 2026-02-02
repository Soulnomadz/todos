use salvo::prelude::*;
use salvo::http::HeaderValue;
use crate::db::get_pgpool;
use crate::types::*;

#[handler]
pub fn index(res: &mut Response) {
    let templates = crate::get_templates();
    let mut context = tera::Context::new();
    context.insert("username", "");
    context.insert("error_msg", "");
    let rendered = templates.render("login.html", &context).unwrap();

    res.render(Text::Html(rendered));     
}

#[handler]
pub async fn list_todos(req: &mut Request, res: &mut Response) {
    let todos: Vec<Todo> = sqlx::query_as!(
        Todo,
        "select id,text,completed from public.todos order by id desc",
    )
        .fetch_all(get_pgpool())
        .await
        .map_err(|e| {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::BAD_REQUEST
        }).unwrap();

    let templates = crate::get_templates();
    let mut context = tera::Context::new();
    context.insert("todos", &todos);
    let rendered = templates.render("todos.html", &context).unwrap();

    res.render(Text::Html(rendered));
} 

#[handler]
pub async fn get_todo_by_id(req: &mut Request, res: &mut Response) {
    let id = match req.param::<i64>("id") {
        Some(id) => id,
        None => {
            tracing::debug!("Error: bad param id");
            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };

    match sqlx::query_as!(
        Todo,
        "select id,text,completed from public.todos where id = $1",
	id
    ).fetch_optional(get_pgpool()).await {
	Ok(Some(todo)) => res.render(Json(todo)),
	Ok(None) => {
	    tracing::debug!("todo not found with id: {}", id);
	    res.render(Json({}));
	},
	Err(e) => {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::INTERNAL_SERVER_ERROR;
	},
    }
}

#[handler]
pub async fn create_todo(req: &mut Request, res: &mut Response) {
    let new_todo: NewTodo = req
        .parse_body_with_max_size(512)
        .await
        .map_err(|e| {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::BAD_REQUEST
        }).unwrap();
    
    let ret = sqlx::query!(
        "insert into public.todos (text) values ($1) returning id",
        new_todo.text,
    )
	.fetch_one(get_pgpool())
        .await
        .map_err(|e| {
            tracing::debug!("Error: {}", e);
            salvo::http::StatusCode::BAD_REQUEST
        }).unwrap();

    tracing::debug!(todo = ?new_todo, "create todo");

    res.render(Text::Plain(ret.id.to_string()));
}

#[handler]
pub async fn update_todo(req: &mut Request, res: &mut Response) {
    let id = match req.param::<i64>("id") {
        Some(id) => id,
        None => {
            tracing::debug!("Error: bad param id");
            res.status_code(StatusCode::BAD_REQUEST);
            return;
        }
    };

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
    let id = match req.param::<i64>("id") {
        Some(id) => id,
        None => {
            tracing::debug!("Error: bad param id");
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

#[handler]
pub async fn login(req: &mut Request, res: &mut Response) {
    let user = req.form::<String>("username").await.unwrap_or_default();
    let pass = req.form::<String>("password").await.unwrap_or_default();

    let user_exist = sqlx::query!(
	"select id from users where name = $1 and pass = $2",
	user,
	pass,
    ).fetch_optional(get_pgpool()).await.unwrap_or_default();

    
    if user_exist.is_some() {
	res.headers_mut().insert(
	    "HX-Redirect",
	    HeaderValue::from_str("/todos").unwrap()
	);
    } else {
	let error_html = "<span>用户名或密码错误，请重新输入！</span>";
        res.render(Text::Html(error_html));
    }
}
