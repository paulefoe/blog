#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::templates::{Template, handlebars};
use serde::{Serialize};
use rocket_contrib::{serve::{StaticFiles}};
use comrak::{markdown_to_html, ComrakOptions};
use rocket_contrib::databases::{diesel, database};

use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult, JsonRender};


mod post;

use post::{Post};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);


#[get("/")]
fn hello(conn: DbConn) -> Template {
    let posts = Post::all(&conn);
    println!("{}", markdown_to_html("# Habits I want to develop as a Software Engineer", &ComrakOptions::default()),);
    Template::render("index", &TemplateContext {
        title: "Hello",
        parent: "layout",
    })
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", &TemplateContext {
        title: "About",
        parent: "layout",
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn wow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("/home/paulefou/other_projects/blog/static"))
        .mount("/", routes![hello, about])
        .register(catchers![not_found])
        .attach(DbConn::fairing())
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
        }))
}

fn main() {
    rocket().launch();
}