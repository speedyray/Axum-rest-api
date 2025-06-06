#![allow(unused)]
use axum::{routing::get,Router};

mod vehicle;

use vehicle::{vehicle_get, vehicle_post};
#[tokio::main]
pub async fn main() {
    //Create the axum router
   let router01 = Router::new()
   .route("/vehicle", get(vehicle_get).post(vehicle_post));
   //Define the IP and port listener (TCP)
   let address = "0.0.0:6570";
   let listener  = tokio::net::TcpListener::bind(address).await.unwrap();

   //Axum serve to launch the web server
   axum::serve(listener, router01).await.unwrap();
}
