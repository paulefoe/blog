#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate blog;

use rocket::Request;
use rocket_contrib::templates::{Template, handlebars};
use serde::{Serialize};
use rocket_contrib::{serve::{StaticFiles}};
use comrak::{markdown_to_html, ComrakOptions};
use rocket_contrib::databases::{diesel, database};
use chrono::{NaiveDateTime};
use rss::{ChannelBuilder, Item};
use rocket::response::content;



use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};


use self::blog::*;
use self::models::*;

#[derive(Serialize)]
struct TemplateListContext {
    title: &'static str,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
    posts: Vec<Post>
}

#[derive(Serialize)]
struct TemplateDetailContext {
    title: &'static str,
    parent: &'static str,
    post: Post,
}

#[database("posts")]
pub struct DbConn(diesel::SqliteConnection);


#[get("/")]
fn index(conn: DbConn) -> Template {
    let posts = Post::all(&conn);
    Template::render("index", &TemplateListContext {
        title: "Blog",
        parent: "layout",
        posts: posts
    })
}

#[get("/blog/rss")]
fn rss(conn: DbConn) -> content::Xml<String> {
    let posts = Post::all(&conn);

    let mut items: Vec<Item> = vec![];
    for post in posts {
        let mut item = Item::default();
        item.set_title(post.title);
        item.set_link(format!("https://paulefou.com/blog/{}", post.slug));
        item.set_description(post.description);
        item.set_author(String::from("Paulefou"));
        item.set_content(markdown_to_html(&post.body, &ComrakOptions::default()));
        &items.push(item);
    }

    let channel = ChannelBuilder::default()
    .title("paulefou blog")
    .link("https://paulefou.com")
    .description("An RSS feed for paulefou blog posts.")
    .items(items)
    .build()
    .unwrap();

    content::Xml(channel.to_string())
}

#[get("/blog/<slug>")]
fn detail(conn: DbConn, slug: String) -> Template {
    let post = Post::detail(&conn, &slug);
    Post::increment_views_count(&conn, &slug);
    Template::render("detail", &TemplateDetailContext {
        title: "Blog",
        parent: "layout",
        post
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", &TemplateListContext {
        title: "About",
        parent: "layout",
        posts: vec![],
    })
}

#[get("/contact")]
fn contact() -> Template {
    Template::render("contact", &TemplateListContext {
        title: "Contact",
        parent: "layout",
        posts: vec![],
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn format_date(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        let date = NaiveDateTime::parse_from_str(
            &param.value().render(),
            "%Y-%m-%dT%H:%M:%S%.9f"
        ).unwrap();

        out.write(format!("{}", date.format("%Y-%m-%d")).as_str())?;
    }

    Ok(())
}

fn markdown_to_html_templates(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write(&*markdown_to_html(&param.value().render(), &ComrakOptions::default()))?;
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("/home/paulefou/other_projects/diesel_demo/static"))
        .mount("/", routes![index, about, detail, contact, rss])
        .register(catchers![not_found])
        .attach(DbConn::fairing())
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("formatDate", Box::new(format_date));
            engines.handlebars.register_helper("formatArticle", Box::new(markdown_to_html_templates));
        }))
}

fn main() {
    rocket().launch();
}
