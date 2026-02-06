use todos::types::*;

fn test_todo() -> Todo {
    Todo {
	id: 4,
	text: "test todo".into(),
	completed: false,
    }
}

#[tokio::test]
async fn test_01_get_todos() -> httpc_test::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8089")?;

    let res = hc.do_get(
	"/todos",
    ).await?;

    //res.print().await?;
    assert_eq!(res.status(), salvo::http::StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_02_crud_todo() -> httpc_test::Result<()> {
    let hc = httpc_test::new_client("http://localhost:8089")?;

    // test create todo
    let res = hc.do_post(
        "/todos",
	serde_json::json!(test_todo()),
    ).await?;

    res.print().await?;
    assert_eq!(res.status(), salvo::http::StatusCode::CREATED);

    let id = res.text_body()?;
    let url = format!("/todos/{id}");

    // test update todo
    let new_todo = Todo {
        id: 4,
        text: "new test todo".into(),
        completed: false,
    };

    let res = hc.do_put(
        &url,
        serde_json::json!(new_todo),
    ).await?;

    res.print().await?;
    assert_eq!(res.status(), salvo::http::StatusCode::OK);

    // test delete todo
    let res = hc.do_delete(
    	&url,
    ).await?;

    res.print().await?;
    assert_eq!(res.status(), salvo::http::StatusCode::NO_CONTENT);

    Ok(())
}
