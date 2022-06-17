#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;


fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db url not found");

    let conn =PgConnection::establish(&db_url).expect("Failed to connect to database");

    use self::models::{Post, NewPost, PostSimple};
    use self::schema::posts;
    use self::schema::posts::dsl::*;


    //CREATING NEW POSTS
    // let new_post = NewPost {
    //     title: "Third Blogpost",
    //     body: "Literally some text",
    //     slug: "third-blogpost"
    // };

    // let post : Post = diesel::insert_into(posts::table).values(&new_post).get_result(&conn).expect("Data Insert Failed");

    //UPDATING POSTS
    //UPDATING TITLE AND SLUG COLUMNS
    //let post_update = diesel::update(posts.filter(id.eq(4))).set((title.eq("Cuarto Blogpost"), slug.eq("fourth-blogpost"))).get_result::<Post>(&conn).expect("Failed to update post");

    //DELETING POSTS
    diesel::delete(posts.filter(id.eq(4))).execute(&conn).expect("Failed to delete post");

    //READING POSTS
    println!("ALL POSTS QUERY:");

    let posts_results = posts.load::<Post>(&conn).expect("Error loading posts");

    for post in posts_results {
        println!("{:?}", post);
    }

    // println!("LIMITED POSTS QUERY");

    // let posts_results = posts.limit(1).load::<Post>(&conn).expect("Error loading posts");

    // for post in posts_results {
    //     println!("{:?}", post);
    // }

    // println!("SELECT COLUMNS QUERY");

    // let posts_results = posts.select((title,body)).load::<PostSimple>(&conn).expect("Error loading posts");

    // for post in posts_results {
    //     println!("{:?}", post);
    // }

    // println!("ORDER BY QUERY:");

    // let posts_results = posts.order(id.desc()).load::<Post>(&conn).expect("Error loading posts");

    // for post in posts_results {
    //     println!("{:?}", post);
    // }

    // println!("FILTER BY WHERE (case body):");

    // let posts_results = posts.filter(body.eq("Literally some text")).load::<Post>(&conn).expect("Error loading posts");

    // for post in posts_results {
    //     println!("{:?}", post);
    // }

    // println!("FILTER BY WHERE (case specific id):");

    // let posts_results = posts.filter(id.eq(2)).load::<Post>(&conn).expect("Error loading posts");

    // for post in posts_results {
    //     println!("{:?}", post);
    // }

    //EDIT POSTS

    
}
