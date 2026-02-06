use thiserror::Error;
use salvo::prelude::*;

#[derive(Error, Debug)]
pub enum TodoError {
    #[error("环境变量错误: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Tera Error: {0}")]
    TeraError(#[from] tera::Error),

    #[error("bad config: {0}")]
    ConfigError(String),

    #[error("Db init error")]
    InitError,
}

#[async_trait]
impl Writer for TodoError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render("Todo error");
    }
}

//impl From<TodoError> for salvo::Error {
//    fn from(err: TodoError) -> Self {
//        anyhow::anyhow!("internal error")
//    }
//}

//impl Into<salvo::Error> for TodoError {
//    fn into(self) -> salvo::Error {
//        // 根据自定义错误类型，返回对应的 salvo 错误和状态码
//        match self {
//            TodoError::SqlxError(e) => {
//                // 数据库错误返回 500 Internal Server Error
//                salvo::Error::new(StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
//            }
//	    _ => salvo::Error::new(StatusCode::INTERNAL_SERVER_ERROR, format!("error 500")),
//        }
//    }
//}
