use chrono::naive::NaiveDateTime;
use super::schema::posts;


#[derive(Queryable)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: Option<NaiveDateTime>,
    pub views_count: Option<i32>,
    pub description: String,
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
}
