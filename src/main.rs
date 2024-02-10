#[cfg(test)]
mod tests;
mod server;

use crate::server::RestaurantApp;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct OrderRequest{
    item: String,
    table_number: u32,
}

async fn add_order(data: web::Data<RestaurantApp>, info: web::Json<OrderRequest>) -> impl Responder {
    println!("Adding order {:?}", info);
    let OrderRequest{item, table_number} = info.into_inner();
    println!("table number : item={}. table={}", item, table_number);
    data.add_order(item, table_number);
    HttpResponse::Ok().body("Order added successfully.")
}

async fn remove_order(data: web::Data<RestaurantApp>, info: web::Json<OrderRequest>) -> impl Responder {
    println!("Removing order {:?}", info);
    let OrderRequest{item, table_number} = info.into_inner();
    data.remove_order(&item, table_number);
    HttpResponse::Ok().body("Order removed successfully.")
}

async fn query_all_orders(data: web::Data<RestaurantApp>, info: web::Json<u32>) -> impl Responder {
    println!("Getting all orders");
    let table_number = info.into_inner();
    let orders = data.query_all_orders(table_number);
    HttpResponse::Ok().json(orders)

    // MOCK DATA
    // let mut j = vec![];
    // j.push(Order{item: "tiepp".to_string(), table_number: 4, cook_time: 2});
    // j.push(Order{item: "couscous".to_string(), table_number: 2, cook_time:2});
    // HttpResponse::Ok().json(j)
}

async fn get_order(data: web::Data<RestaurantApp>, info: web::Json<OrderRequest>) -> impl Responder {
    println!("Getting order {:?}", info);
    let OrderRequest{item, table_number} = info.into_inner();
    let order = data.get_order(&item, table_number);
    HttpResponse::Ok().json(order)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = RestaurantApp::new();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app.clone()))
            .route("/add_order", web::post().to(add_order))
            .route("/remove_order", web::post().to(remove_order))
            .route("/query_all_orders", web::get().to(query_all_orders))
            .route("/get_order", web::get().to(get_order))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}