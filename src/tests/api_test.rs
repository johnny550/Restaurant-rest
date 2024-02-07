use crate::{get_order, query_all_orders, add_order, remove_order, server::{Order, RestaurantApp}, OrderRequest};

// use crate::server::RestaurantApp;
// use actix_rt::test;
// use actix_web::{http, web};
// use actix_web::{HttpResponse, Responder};
// use actix_web::{http::StatusCode, web};


use actix_web::{http::{self, header::{self, HeaderValue}}, web, test, App, HttpResponse};
use serde_json::{json, Value}; // Import serde_json for JSON parsing
// #[test]
#[actix_rt::test]
async fn test_query_all_orders() {
    // Setup
    let resto: RestaurantApp = RestaurantApp::new();
    let data = web::Data::new(resto.clone());

     // Create a test app with the handler function: query_all_orders
     let mut app = test::init_service(App::new()
     .app_data(data)
     .route("/all_orders", web::get().to(query_all_orders))
    ).await;

     // Create a test request to the /all_orders endpoint
     let req = test::TestRequest::get()
     .uri("/all_orders")
     .set_json(2)
     .insert_header((header::CONTENT_TYPE, HeaderValue::from_static("application/json")))
     .to_request();

     // Send the request to the app and get the response
     let response = test::call_service(&mut app, req).await;
    
     // Check if the response status is OK (200)
     assert_eq!(response.status(), http::StatusCode::OK, "Returned status code should be 200");

         // figure out how to extract the content of the body
    //  let body = test::read_body(response).await;
    //  let orders = match str::from_utf8(&body) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    // println!("body {:?}", orders);
    // print_type_of(&orders);


    // let retrieved: Result<OrderRequest, serde_json::error::Error> = serde_json::from_str(orders);
    // println!("rst: {:?}", retrieved);
    
    // assert!(!orders.is_empty(), "Response body could not be parsed");
    // assert!(retrieved.is_ok(), "Response body could not be parsed");
    

    // let orders = retrieved.unwrap();

    // let retrieved= test::read_body(response).await;
    
    // println!("body {:?}", body);

 
}

#[actix_rt::test]
async fn test_get_order() {
    // Setup
    let resto = RestaurantApp::new();
    let data = web::Data::new(resto.clone());

     // Create a test app with the handler function: query_all_orders
     let mut app = test::init_service(App::new()
     .app_data(data)
     .route("/order", web::get().to(get_order))
    ).await;

    // Create a test request to the /all_orders endpoint
    let payload = json!({"item": "mesazza", "table_number":2});
    
    let req = test::TestRequest::get()
    .uri("/order")
    .insert_header((header::CONTENT_TYPE, HeaderValue::from_static("application/json")))
    .set_payload(payload.to_string())
    .to_request();

    println!("req headers {:?}", req.head());

    // Perform the test
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_add_order() {
    // Setup
    let resto = RestaurantApp::new();
    let data = web::Data::new(resto.clone());

     // Create a test app with the handler function: query_all_orders
     let mut app = test::init_service(App::new()
     .app_data(data)
     .route("/add_order", web::post().to(add_order))
    ).await;

    // Create a test request to the /all_orders endpoint
    let payload = json!({"item": "mesazza", "table_number":2});
    let req = test::TestRequest::post()
    .uri("/add_order")
    .insert_header((header::CONTENT_TYPE, HeaderValue::from_static("application/json")))
    .set_payload(payload.to_string())
    .to_request();

    // println!("req headers {:?}", req.head());

    // Perform the test
    let response = test::call_and_read_body(&mut app, req).await;

    // Assert
    assert_eq!(response, "Order added successfully.".to_string(), "Unexpected output");
}

#[actix_rt::test]
async fn test_delete_order() {
    // Setup
    let resto = RestaurantApp::new();
    let data = web::Data::new(resto.clone());

     // Create a test app with the handler function: query_all_orders
     let mut app = test::init_service(App::new()
     .app_data(data)
     .route("/remove_order", web::post().to(remove_order))
    ).await;

    // Create a test request to the /all_orders endpoint
    let payload = json!({"item": "mesazza", "table_number":2});
    
    let req = test::TestRequest::post()
    .uri("/remove_order")
    .insert_header((header::CONTENT_TYPE, HeaderValue::from_static("application/json")))
    .set_payload(payload.to_string())
    .to_request();

    println!("req headers {:?}", req.head());

    // Perform the test
    let response = test::call_and_read_body(&mut app, req).await;

    // Assert
    assert_eq!(response, "Order removed successfully.".to_string(), "Unexpected output");
}
