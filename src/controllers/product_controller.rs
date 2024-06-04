use crate::database::get_collection;
use crate::models::product_model::Product;
use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{Client, Collection};

const COLLECTION_NAME: &str = "products";

pub async fn get_products(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Product> = get_collection(&client, COLLECTION_NAME);

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
    let collection: Collection<Product> = get_collection(&client, COLLECTION_NAME);

    let new_product = Product::new(
        item.name.clone(),
        item.description.clone(),
        item.price,
        item.category.clone(),
        item.user,
        item.images.clone(),
        Some(item.stock),
    );

    let insert_result = collection.insert_one(new_product, None).await;

    match insert_result {
        Ok(inserted) => HttpResponse::Ok().json(inserted.inserted_id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
