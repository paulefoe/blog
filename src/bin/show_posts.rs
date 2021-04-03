extern crate blog;
extern crate diesel;

use self::blog::*;
use self::models::*;
use self::diesel::prelude::*;

#[allow(dead_code)]
fn main() {
    use blog::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}