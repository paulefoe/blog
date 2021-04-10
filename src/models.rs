use super::schema::posts::dsl::*;
use chrono::naive::NaiveDateTime;
use super::schema::posts;
use serde::{Serialize};
use crate::diesel::query_dsl::limit_dsl::LimitDsl;
use crate::diesel::RunQueryDsl;
use diesel::sqlite::SqliteConnection;
use crate::diesel::query_dsl::filter_dsl::FilterDsl;
use crate::diesel::ExpressionMethods;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub views_count: Option<i32>,
    pub description: String,
    pub slug: String,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub views_count: i32,
    pub description: &'a str,
    pub slug: &'a str
}

impl Post {
    pub fn all(conn: &SqliteConnection) -> Vec<Post> {
        posts::dsl::posts
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts")
    }

    pub fn detail(conn: &SqliteConnection, other_slug: &String) -> Post {
        let a: Post = posts::dsl::posts.filter(slug.eq(other_slug)).first(conn).expect("Did'n find the post");
        a
    }

    pub fn increment_views_count(conn: &SqliteConnection, other_slug: &String) {
        // let r = diesel::update(posts).set(views_count.eq(views_count + 1));
        // println!("{:?}", r);
        diesel::update(posts.filter(slug.eq(other_slug))).set(views_count.eq(views_count + 1)).execute(conn);
    }
}