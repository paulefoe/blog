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

        // posts::dsl::posts.filter(title.eq(1)).first(conn).expect("Did'n find the post")
    let a: Post = posts.filter(slug.eq("slug")).first(&connection).expect("fuck you");

    println!("Displaying {:?} posts", a.body);
    // for post in results {
    //     println!("{}", post.title);
    //     println!("----------\n");
    //     println!("{}", post.body);
    // }
}