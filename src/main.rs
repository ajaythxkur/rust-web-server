use actix_web::{App, HttpResponse, HttpServer, Responder, get, web::Data};

use crate::{
    routes::{
        booking_route::{cancel_booking, create_booking, get_bookings},
        dog_route::create_dog,
        owner_route::create_owner,
    },
    services::db::Database,
};

mod models;
mod routes;
mod services;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("woof woof")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let db = Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_owner)
            .service(create_dog)
            .service(create_booking)
            .service(get_bookings)
            .service(cancel_booking)
    })
    .bind(("127.0.0.1", 8585))?
    .run()
    .await
}
