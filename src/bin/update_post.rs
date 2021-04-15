extern crate blog;
extern crate diesel;

use self::blog::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What is the slug of the post that you want to update?");
    let mut slug = String::new();
    stdin().read_line(&mut slug).unwrap();
    let slug = &slug[..(slug.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's show me the new body for {} (Press {} when finished)\n", &slug, EOF);

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    println!("{} fucking slug here?", &slug);

    let _post = update_post(&connection, &body, &slug);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";