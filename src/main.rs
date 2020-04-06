extern crate todo_list_diesel;
extern crate diesel;

use todo_list_diesel::models::*;
use self::diesel::prelude::*;
use std::io::stdin;

fn new_post() {
    let connection = Database::establish_connection();

    println!("What do you want to insert?");
    let mut new_title = String::new();
    stdin().read_line(&mut new_title).unwrap();
    new_title = new_title[..(new_title.len() - 1)].to_string();

    let _post = Database::new_post(&connection, &new_title);
    println!("Saved.");

}

fn get_post() {
    use todo_list_diesel::schema::todo::dsl::*;

    let connection = Database::establish_connection();
    let results = todo
        .limit(5)
        .load::<Database>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("ID:{} - [{}] - title: {}", post.id, post.finished, post.title);
    }
}

fn delete_post() {
    let connection = Database::establish_connection();

    println!("What do you want to delete?");
    let mut delete_this = String::new();
    stdin().read_line(&mut delete_this).unwrap();
    let delete_this = delete_this.trim().parse::<i32>().unwrap();
    
    Database::delete_post(&connection, delete_this);
}

fn complete_post() {

    let connection = Database::establish_connection();

    println!("What task to set finished/unfinished?");
    let mut update_this = String::new();
    stdin().read_line(&mut update_this).unwrap();
    let update_this = update_this.trim().parse::<i32>().unwrap();
    
    Database::complete_post(&connection, update_this);
}

fn main() {
    // test commands for now.
    get_post();
    new_post();
    complete_post();
    delete_post();
}