#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use self::models::{Post, NewPost, NewPostHandler};
use self::schema::posts;
use self::schema::posts::dsl::*;

use dotenv::dotenv;
use std::env;
use tera::Tera;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[get("/")]
async fn index(pool:  web::Data<DbPool>, template_manager : web::Data<tera::Tera>) -> impl Responder {
    let conn = pool.get().expect("Failed to get connection from database");

    match web::block(move || {posts.load::<Post>(&conn)}).await {
        Ok(data) => {

            let data = data.unwrap();

            let mut ctx = tera::Context::new();

            ctx.insert("posts", &data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("index.html", &ctx).unwrap()
            )
        },
        Err(err) => HttpResponse::Ok().body("Error loading data"),
    }
}


#[get("/blog/{blog_slug}")]
async fn get_post(
    pool:  web::Data<DbPool>, 
    template_manager : web::Data<tera::Tera>,
    blog_slug : web::Path<String>
) -> impl Responder { 
    let conn = pool.get().expect("Failed to get connection from database");

    let url_slug = blog_slug.into_inner();

    match web::block(move || {posts.filter(slug.eq(url_slug)).load::<Post>(&conn)}).await {
        Ok(data) => {

            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().finish();
            }

            let data = &data[0];

            let mut ctx = tera::Context::new();

            ctx.insert("post", &data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("posts.html", &ctx).unwrap()
            )
        },
        Err(err) => HttpResponse::Ok().body("Error loading data"),
    }
}


#[get("/tera_test")]
async fn tera_test(template_manager : web::Data<tera::Tera>) -> impl Responder{
    
    let mut ctx = tera::Context::new();
    
    HttpResponse::Ok().content_type("text/html").body(
        template_manager.render("index.html", &ctx).unwrap()
    )
}



#[post("/new_post")]
async fn new_post(pool:  web::Data<DbPool>, item : web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Failed to get connection from database");

    match web::block(move || {Post::create_post(&conn, &item)}).await {
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => HttpResponse::Ok().body("Error loading data"),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url not found");

    let port = env::var("PORT").expect("db url not found");
    let port : u16 = port.parse().unwrap();

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder().build(connection).expect("Pool Construction went wrong");

    HttpServer::new(move || {

        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
        .service(index)
        .service(new_post)
        .service(tera_test)
        .service(get_post)
        .data(pool.clone())
        .data(tera)
    }).bind(("0.0.0.0", port)).unwrap().run().await
}
