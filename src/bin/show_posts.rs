extern crate blog;
extern crate diesel;

use self::blog::*;
use self::models::*;
use self::diesel::prelude::*;
use blog::schema::posts::dsl::*;

fn main() {
    //

    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    let a: Post = posts.filter(slug.eq("slug")).first(&connection).expect("fuck you");

    println!("Displaying {:?} posts", a.body);
}