use axum::Json;
use axum::debug_handler;



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle{
    manufacturer:String,
    model:String,
    year:u32,
    id: Option<String>,
}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehicle>{
    Json::from(Vehicle {
        manufacturer: "Dodge".to_string(),
        model: "RAM".to_string(),
        year: 2001,
        id: Some(uuid::Uuid::new_v4().to_string()),

    })

}

pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle>{
  println!("Manufacturer:{0},model: {1}, year: {2}", v.manufacturer, v.model, v.year );
  v.id = Some(uuid::Uuid::new_v4().to_string());
  Json::from(v)
}
