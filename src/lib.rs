#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Database, NewPost};

impl Database {

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();
    
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    pub fn new_post<'a>(conn: &PgConnection, title: &'a str) -> Database {
        use schema::todo as database;
    
        let new_post = NewPost {
            title: title
        };
    
        diesel::insert_into(database::table)
            .values(&new_post)
            .get_result(conn)
            .expect("Error saving new post")
    }
    
    pub fn delete_post(conn: &PgConnection, post_id: i32){
        use schema::todo as database;

        // TODO: Should probably give some feedback if post_id doesn't exist.    
        let result = diesel::delete(database::table.find(post_id))
            .get_result::<Database>(conn);
        
        if result.is_err() {
            println!("ID does not exist.")
        } else {
            println!("Deleted.")
        }

    }
    
    pub fn complete_post(conn: &PgConnection, post_id: i32) {
            use schema::todo as database;
            use schema::todo::dsl::*;
            
            let post = todo.filter(id.eq(post_id))
                .load::<Database>(conn)
                .unwrap();
    
            let mut value = false;
    
            for p in post {
                if p.finished.eq(&true) {
                    value = false;
                } else {
                    value = true;
                }
            }
    
            diesel::update(database::table.find(post_id))
                .set(finished.eq(&value))
                .execute(conn)
                .expect("Could not update database");
    }
}