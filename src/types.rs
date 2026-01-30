use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewTodo {
    pub text: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

