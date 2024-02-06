use actix_web::{HttpResponse, Responder};

pub async fn test_endpoint() ->
    impl Responder{
        HttpResponse::Ok().body("Test endpoint works. ")
    }
