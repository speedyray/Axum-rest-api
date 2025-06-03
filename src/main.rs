#![allow(unused)]
use axum::{
    debug_handler, routing::{get, post}, Json, Router
};

#[tokio::main]
async fn main() {
    //Create the axum router
   let router01 = Router::new()
   .route("/vehicle", get(vehicle_get).post(vehicle_post));
   //Define the IP and port listener (TCP)
   let address = "0.0.0:6570";
   let listener  = tokio::net::TcpListener::bind(address).await.unwrap();

   //Axum serve to launch the web server
   axum::serve(listener, router01).await.unwrap();
}
#[derive(Debug, serde::Serialize)]
struct Vehicle{
    manufacturer:String,
    model:String,
    year:u32,
    id:String,
}

#[debug_handler]
async fn vehicle_get() -> Json<Vehicle>{
    Json::from(Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RAM".to_string(),
        year: 2001,
        id:uuid::Uuid::new_v4().to_string(),

    })

}


async fn vehicle_post(){

}
