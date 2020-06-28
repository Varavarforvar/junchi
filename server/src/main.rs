#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, Error };
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use actix::prelude::*;
extern crate dotenv;
use std::env;

struct DbExecutor(MysqlConnection);

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

impl Actor for DbExecutor {
  type Context = SyncContext<Self>;
}

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();

  let mut listenfd = ListenFd::from_env();
  let connspec = env::var("DATABASE_URL").expect("DATABASE_URL");
  let manager = ConnectionManager::<MysqlConnection>::new(connspec);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");

  let mut server = HttpServer::new(move || {
    App::new()
      .data(pool.clone())
  }
    .route("/", web::get().to(index))
  );

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l)?
  } else {
    server.bind("127.0.0.1:3000")?
  };
  server.run().await
}
