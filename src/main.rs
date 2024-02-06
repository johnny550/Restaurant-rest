mod ep_test;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize)]
struct Order {
    item: String,
    table_number: u32,
    cook_time: u32,
}

#[derive(Clone)]
struct RestaurantApp {
    orders: Arc<Mutex<HashMap<u32, HashSet<Order>>>>,
}

impl RestaurantApp {
    fn new() -> Self {
        RestaurantApp {
            orders: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_order(&self, item: String, table_number: u32) {
        println!("Adding order");
        let cook_time = rand::thread_rng().gen_range(5..=15);
        let order = Order {
            item,
            table_number,
            cook_time,
        };

        let mut orders = self.orders.lock().unwrap();
        orders
            .entry(table_number)
            .or_insert_with(HashSet::new)
            .insert(order);
    }

    fn remove_order(&self, item: &str, table_number: u32) {
        let mut orders = self.orders.lock().unwrap();
        if let Some(table_orders) = orders.get_mut(&table_number) {
            table_orders.retain(|order| order.item != item);
            if table_orders.is_empty() {
                orders.remove(&table_number);
            }
        }
    }

    fn query_all_orders(&self, table_number: u32) -> Vec<Order> {
        let orders = self.orders.lock().unwrap();
        orders.get(&table_number).cloned().unwrap_or_default().into_iter().collect()
    }

    fn get_order(&self, item: &str, table_number: u32) -> Option<Order> {
        let orders = self.orders.lock().unwrap();
        orders
            .get(&table_number)
            .and_then(|table_orders| table_orders.iter().find(|order| order.item == item).cloned())
    }
}

#[derive(Debug, Deserialize)]
struct OrderRequest{
    item: String,
    table_number: u32,
}

// async fn add_order(data: web::Data<RestaurantApp>, info: web::Json<(String, u32)>) -> impl Responder {
async fn add_order(data: web::Data<RestaurantApp>, info: web::Json<OrderRequest>) -> impl Responder {
    println!("Adding order {:?}", info);
    let OrderRequest{item, table_number} = info.into_inner();
    println!("table number : item={}. table={}", item, table_number);
    data.add_order(item, table_number);
    HttpResponse::Ok().body("Order added successfully.")
}

async fn remove_order(data: web::Data<RestaurantApp>, info: web::Json<OrderRequest>) -> impl Responder {
    println!("removing order {:?}", info);
    let OrderRequest{item, table_number} = info.into_inner();
    data.remove_order(&item, table_number);
    HttpResponse::Ok().body("Order removed successfully.")
}

async fn query_all_orders(data: web::Data<RestaurantApp>, info: web::Json<u32>) -> impl Responder {
    println!("Getting all orders");
    let table_number = info.into_inner();
    let orders = data.query_all_orders(table_number);
    HttpResponse::Ok().json(orders)
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
            .route("/test_endpoint", web::get().to(ep_test::test_endpoint))
            .route("/add_order", web::post().to(add_order))
            .route("/remove_order", web::post().to(remove_order))
            .route("/query_all_orders", web::get().to(query_all_orders))
            .route("/get_order", web::get().to(get_order))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



// use actix_web::{web, App, HttpResponse, HttpServer};

// async fn test_endpoint() -> HttpResponse {
//     HttpResponse::Ok().body("Test endpoint works.")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().route("/test_endpoint", web::get().to(test_endpoint))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }