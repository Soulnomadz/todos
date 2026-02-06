use salvo::prelude::*;
use salvo::http::HeaderValue;
use salvo::session::Session;
use tera::Context;
use anyhow::anyhow;
use tokio::time::timeout;

use std::time::Duration;

use crate::{get_pgpool, TEMPLATES};
use crate::types::*;
use crate::error::TodoError;

fn render_template(res: &mut Response, template_name: &str, context: &Context) {
    match TEMPLATES.render(template_name, context) {
        Ok(rendered) => res.render(Text::Html(rendered)),
        Err(e) => {
            tracing::debug!("tera template render error: {e}");

            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Text::Plain("internal server error"));
        }
    } 
}

#[handler]
pub async fn index(res: &mut Response) {
    let mut context = Context::new();
    context.insert("username", "");
    context.insert("error_msg", "");

    render_template(res, "login.html", &context);
}

#[handler]
pub async fn list_todos(req: &mut Request, depot: &mut Depot, res: &mut Response) -> Result<(), TodoError> {
    let session = match depot.session() {
	Some(s) => s,
	None => {
    	    res.render(Redirect::found("/"));
	    return Ok(());
	}
    };

    let username = match session.get::<String>("username") {
	Some(name) => name,
	None => {
	    tracing::warn!("获取用户名失败或用户名不存在");
    	    res.render(Redirect::found("/"));
	    return Ok(());
	}
    };

    //let todos: Vec<Todo> = match sqlx::query_as!(
    let todos: Vec<Todo> = sqlx::query_as!(
        Todo,
        "select id,text,completed from public.todos order by id desc",
    )
    .fetch_all(get_pgpool())
    .await
    .map_err(|e| {
	tracing::debug!("database error: {e}");

	TodoError::SqlxError(e)
    })?;

    let mut context = Context::new();
    context.insert("username", &username);
    context.insert("todos", &todos);

    render_template(res, "todos.html", &context);

    Ok(())
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

            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
	},
    }
}

#[handler]
pub async fn create_todo(req: &mut Request, res: &mut Response) {
    let new_todo = match req
        .parse_body_with_max_size::<NewTodo>(512)
        .await
    {
	Ok(todo) => todo,
	Err(e) => {
            tracing::debug!("Error: {}", e);

            res.status_code(StatusCode::BAD_REQUEST);
	    return;
	}
    };

    let result = match sqlx::query!(
        "insert into public.todos (text) values ($1) returning id",
        new_todo.text,
    ).fetch_one(get_pgpool()).await
    {
	Ok(row) => row,
	Err(e) => {
            tracing::debug!("Error: {}", e);

            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
	    return;
	} 
    };

    tracing::debug!(todo = ?new_todo, "create todo");

    res.status_code(StatusCode::CREATED);
    res.render(Text::Plain(result.id.to_string()));
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

    let todo: Todo = match req.parse_body_with_max_size(512).await {
	Ok(row) => row,
        Err(e) => {
            tracing::debug!("Error: {}", e);
            
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };

    let result = match sqlx::query!(
        "update public.todos set text = $1, completed = $2 where id = $3",
        todo.text,
        todo.completed,
        id,
    )
        .execute(get_pgpool())
        .await
    {
	Ok(ret) => ret,
        Err(e) => {
            tracing::debug!("Error: {}", e);

            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };

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

    match sqlx::query!(
        "delete from public.todos where id = $1",
        id,
    ).execute(get_pgpool()).await 
    {
	Ok(result) => {
    	    if result.rows_affected() == 0 {
        	tracing::debug!("Error: id not found！");

        	res.status_code(StatusCode::BAD_REQUEST);
    	    } else {
    	        tracing::debug!(id = id, "deleted: ");

        	res.status_code(StatusCode::NO_CONTENT);
    	    }
	}

	Err(e) => {
    	    tracing::debug!("Error: {e}");

            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
	}
    }
}

#[handler]
pub async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let user = req.form::<String>("username").await.unwrap_or_default();
    let pass = req.form::<String>("password").await.unwrap_or_default();

    let user_record = sqlx::query!(
	"select pass from users where name = $1",
	user,
    )
    .fetch_optional(get_pgpool()).await;
    
    match user_record {
	Ok(Some(record)) => {
	    if bcrypt::verify(pass, &record.pass).unwrap() {
		let mut session = Session::new();
		session.insert(
		    "username", 
		    &user
		).unwrap();
		depot.set_session(session);

		res.headers_mut().insert(
		    "HX-Redirect",
		    HeaderValue::from_str("/todos").unwrap(),
		);

		tracing::debug!(user = ?user, "user logged:");
	    } else {
		let error_html = "<span>密码错误，请重新输入！</span>";
        	res.render(Text::Html(error_html));
	    }
	}
	Ok(None) => {
	    let error_html = "<span>用户名不存在，请重新输入！</span>";
            res.render(Text::Html(error_html));
	}
	Err(e) => {
	    tracing::debug!("sql执行失败");

	    let error_html = "<span>服务器内部错误！</span>";
            res.render(Text::Html(error_html));
	}
    }
}

#[handler]
pub async fn logout(depot: &mut Depot, res: &mut Response) {
    if let Some(session) = depot.session_mut() {
        session.remove("username");
    }
    res.render(Redirect::other("/"));
}

#[handler]
pub async fn register(req: &mut Request, res: &mut Response) {
    let bcrypt_cost: u32 = std::env::var("BCRYPT_COST")
	.unwrap_or("10".to_string())
	.parse()
	.unwrap_or(10);

    let user = req.form::<String>("username").await;
    let pass = req.form::<String>("password").await;
    let pass2 = req.form::<String>("confirm_password").await;

    if let (Some(user), Some(pass), Some(pass2)) = (user, pass, pass2) {
	if pass == pass2 {
	    let pass_hash = bcrypt::hash(pass, bcrypt_cost).unwrap();
	    //let pass_hash = "123456";
	    match sqlx::query!(
	        "insert into users (name, pass) values ($1, $2)",
	        user,
	        pass_hash,
	    )
	    .execute(get_pgpool())
	    .await {
		Ok(result) => {
		    tracing::debug!("{:?}", result);

	    	    if result.rows_affected() == 1 {
	    		tracing::debug!("user added");

	    		// 注册成功时，返回该 HTML 片段（替换原有表单）
	    		let success_html = r#"
	    		<div class="success-tip" style="text-align: center; padding: 24px 0;">
	    		    <h3 style="color: #52c41a; margin-bottom: 16px;">🎉 注册成功！</h3>
	    		    <p style="color: #666; font-size: 14px; margin-bottom: 20px;">
	    		        将在 <span id="countdown" style="color: #1677ff; font-weight: 600;">3</span> 秒后自动跳转到登录页面...
	    		    </p>
	    		    <p style="font-size: 13px; color: #999;">
	    		        若未自动跳转，请 <a href="/" style="color: #1677ff; text-decoration: none;">点击此处</a>
	    		    </p>
	    		</div>
	    		
	    		<script>
	    		    // 1. 倒计时逻辑
	    		    let countdown = 3;
	    		    const countdownElement = document.getElementById('countdown');
	    		    
	    		    const timer = setInterval(() => {
	    		        countdown--;
	    		        countdownElement.innerText = countdown;
	    		        
	    		        // 2. 倒计时结束，自动跳转
	    		        if (countdown <= 0) {
	    		            clearInterval(timer);
	    		            window.location.href = '/';
	    		        }
	    		    }, 1000);
	    		</script>
	    		"#;
	    		
	    		// 直接返回该 HTML 片段（Htmx 会自动替换容器内容）
	    		res.render(Text::Html(success_html));
	    	    }
		}
	    	Err(e) => {
	    	    tracing::debug!("Error: {}", e);

	    	    let error_html = r#"<span>用户名已存在，请更换用户名重新注册</span>"#;
            	    res.render(Text::Html(format!(
            	        r#"<div class="error-tip has-error">{}</div>"#,
            	        error_html
            	    )));
	    	}
	    }
	}
    }
}

#[handler]
pub async fn show_register_page(res: &mut Response) {
    let mut context = tera::Context::new();
    context.insert("username", "");
    context.insert("error_msg", "");
    
    render_template(res, "register.html", &context);
}

#[handler]
pub async fn timeout_middleware(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    const TIMEOUT_SECS: u64 = 3;
    let timeout_duration = Duration::from_secs(TIMEOUT_SECS);

    tokio::select! {
        // 分支1：执行后续的请求处理逻辑（比如hello处理器）
        _ = ctrl.call_next(req, depot, res) => {
            tracing::info!("请求处理完成 ");
        },
        // 分支2：超时触发
        _ = tokio::time::sleep(timeout_duration) => {
            tracing::error!(
		"timeout here"
            );
            
            res.status_code(salvo::http::StatusCode::REQUEST_TIMEOUT);

	    if req.headers().contains_key("Hx-Request") {
                res.headers_mut().insert("HX-Status", "408".parse().unwrap());
                res.headers_mut().insert("HX-Redirect", "/static/404.html".parse().unwrap());
            }

	    res.render(Redirect::found("/static/404.html"));
            
            ctrl.skip_rest();
        }
    }
}
