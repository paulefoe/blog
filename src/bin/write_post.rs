extern crate blog;
extern crate diesel;

use self::blog::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character

    println!("What would you like your description to be?");
    let mut description = String::new();
    stdin().read_line(&mut description).unwrap();
    let description = &description[..(description.len() - 1)];

    println!("What would you like your slug to be?");
    let mut slug = String::new();
    stdin().read_line(&mut slug).unwrap();
    let slug = &slug[..(slug.len() - 1)];

    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let _post = create_post(&connection, title, &body, &description, &slug);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";