use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Product {
    name: String,
    price: f64,
    description: Option<String>,
}

pub async fn get_products(client: web::Data<Client>) -> impl Responder {
    let db = client.database("db_rustycart");
    let collection = db.collection::<Product>("products");

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut products = Vec::new();

    while let Some(product) = cursor.next().await {
        match product {
            Ok(prod) => products.push(prod),
            Err(_) => return HttpResponse::InternalServerError().finish(),
        }
    }

    HttpResponse::Ok().json(products)
}

pub async fn add_product(client: web::Data<Client>, item: web::Json<Product>) -> impl Responder {
    let db = client.database("db_rustycart");
    let collection = db.collection("products");

    let new_product = Product {
        name: item.name.clone(),
        price: item.price,
        description: item.description.clone(),
    };

    let insert_result = collection.insert_one(new_product, None).await;

    match insert_result {
        Ok(inserted) => HttpResponse::Ok().json(inserted.inserted_id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
