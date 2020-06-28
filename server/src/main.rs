use actix_web::{web, App, HttpServer, HttpResponse, Responder, get };
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};

extern crate diesel;

use server::establish_connection;
use server::models::{ Post, NewPost };
use server::schema::*;
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
struct Posts {
  posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
struct Create {
  title: &'static str,
  body: &'static str,
}

#[derive(Serialize, Deserialize)]
struct Success {
  status: bool,
}

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

async fn get_posts() -> impl Responder {
  let connection = establish_connection();

  use server::schema::posts::dsl::*;
  let results = posts.filter(published.eq(true))
    .limit(5)
    .load::<Post>(&connection)
    .expect("Error loading posts");

  HttpResponse::Ok().json(Posts {
    posts: results,
  })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let mut listenfd = ListenFd::from_env();

  let mut server = HttpServer::new(|| App::new()
    .route("/", web::get().to(index))
    .route("/posts", web::get().to(get_posts))
  );

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l)?
  } else {
    server.bind("127.0.0.1:3000")?
  };

  server.run().await
}
