use diesel::prelude::*;
use serde::{Serialize,Deserialize};

use crate::schema::{dessins, rooms};

#[derive(Queryable, Serialize, Identifiable, Clone)]
#[diesel(table_name = dessins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QueryDessin {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub done: bool,
    pub room_id: i32,
    pub content: Option<Vec<u8>>,
    pub last_request: Option<std::time::SystemTime>,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = rooms)]
pub struct QueryRoom {
    pub id: i32,
    pub name: String,
    pub rules: Option<String>,
    pub is_public: bool,
    pub password_protected: bool,
    pub hashed_password: Option<String>,
    pub colors: Vec<Option<String>>,
    pub resolution: i32,
    pub number_of_dessin: Option<i32>,
    pub created_at: std::time::SystemTime,
    pub hashed_admin: String
}

#[derive(Insertable,Deserialize)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertRoom {
    pub name: String,
    pub rules: Option<String>,
    pub is_public: bool,
    pub password_protected: bool,
    pub hashed_password: Option<String>,
    pub colors: Vec<Option<String>>,
    pub resolution: i32,
    pub number_of_dessin: Option<i32>,
    pub hashed_admin: String,
}

#[derive(Insertable)]
#[diesel(table_name = dessins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertNewDessin {
    pub x: i32,
    pub y: i32,
    pub room_id: i32,
    pub done: bool,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = dessins)]
pub struct ValidateDessin {
    pub done: bool,
    pub content: Option<Vec<u8>>,
}

#[derive(AsChangeset)]
#[diesel(table_name = dessins)]
pub struct RequestDessin {
    pub last_request: Option<std::time::SystemTime>,
}

