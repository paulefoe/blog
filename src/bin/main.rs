#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate blog;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use serde::{Serialize};
use rocket_contrib::{serve::{StaticFiles}};
use comrak::{markdown_to_html, ComrakOptions};
use rocket_contrib::databases::{diesel, database};
use chrono::{NaiveDateTime};

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
    // println!("{}", markdown_to_html("# Habits I want to develop as a Software Engineer", &ComrakOptions::default()),);
    Template::render("index", &TemplateListContext {
        title: "Blog",
        parent: "layout",
        posts: posts
    })
}

#[get("/blog/<slug>")]
fn detail(conn: DbConn, slug: String) -> Template {
    let post = Post::detail(&conn, &slug);
    Post::increment_views_count(&conn, &slug);
    // println!("{}", markdown_to_html("# Habits I want to develop as a Software Engineer", &ComrakOptions::default()),);
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
        // out.write(&param.value().render())?;
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
        out.write(&*markdown_to_html(&param.value().render(), &ComrakOptions::default()));
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("/home/paulefou/other_projects/blog/static"))
        .mount("/", routes![index, about, detail, contact])
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