pub mod schema;
pub mod models;
use chrono::Utc;

// use self::models::{Post, NewPost};
use self::models::{NewPost};


#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(
    conn: &SqliteConnection, 
    title: &'a str,
    body: &'a str,
    description: &'a str,
    ) -> QueryResult<usize> {
    use schema::posts;
    let now = Utc::now();
    let new_post = NewPost {
        title: title,
        body: body,
        created_at: now.naive_utc(),
        description: description,
        published: false,
        views_count: 0,
        slug: "slug",
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
}