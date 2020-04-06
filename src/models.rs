#[derive(Queryable, Identifiable)]
#[table_name="todo"]
pub struct Database {
    pub id: i32,
    pub title: String,
    pub finished: bool,
}

use super::schema::todo;

#[derive(Insertable)]
#[table_name="todo"]
pub struct NewPost<'a> {
    pub title: &'a str,
}