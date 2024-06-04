use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub public_id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Review {
    pub user: ObjectId,
    pub name: String,
    pub rating: f64,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub price: f64,
    #[serde(default = "default_ratings")]
    pub ratings: f64,
    pub images: Vec<Image>,
    pub category: String,
    #[serde(default = "default_stock")]
    pub stock: i32,
    #[serde(default = "default_num_of_reviews")]
    pub num_of_reviews: i32,
    #[serde(default)]
    pub reviews: Vec<Review>,
    pub user: ObjectId,
    #[serde(default = "default_created_at")]
    pub created_at: DateTime<Utc>,
}

impl Product {
    pub fn new(
        name: String,
        description: String,
        price: f64,
        category: String,
        user: ObjectId,
        images: Vec<Image>,
        stock: Option<i32>,
    ) -> Self {
        Product {
            id: None,
            name,
            description,
            price,
            ratings: default_ratings(),
            images,
            category,
            stock: stock.unwrap_or(default_stock()),
            num_of_reviews: default_num_of_reviews(),
            reviews: Vec::new(),
            user,
            created_at: default_created_at(),
        }
    }
}

fn default_ratings() -> f64 {
    0.0
}

fn default_stock() -> i32 {
    1
}

fn default_num_of_reviews() -> i32 {
    0
}

fn default_created_at() -> DateTime<Utc> {
    Utc::now()
}
