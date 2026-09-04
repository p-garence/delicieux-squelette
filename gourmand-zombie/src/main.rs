mod database;

use std::env;

use actix_cors::Cors;
use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use diesel::{r2d2, PgConnection};
use env_logger::Env;
use gourmand_zombie::models::{InsertRoom, QueryDessin};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

extern crate dotenv;

use dotenv::dotenv;

use crate::database::{
    add_adjacent_cell, check_cell, create_first_cell, find_cell, get_available_cells,
    get_side_cells, request_cell, validate_cell,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

// Rooms

#[post("/get_rooms")]
async fn get_rooms(pool: web::Data<database::DbPool>) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| e.to_string())?;

        database::get_rooms(&mut conn)
    })
    .await;
    match result {
        Err(e) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Err(e)) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Ok(rooms)) => HttpResponse::Ok().body(serde_json::to_string(&rooms).unwrap()),
    }
}
#[derive(Deserialize, Clone)]

struct AddRoomRequest {
    name: String,
    rules: Option<String>,
    is_public: bool,
    password_protected: bool,
    colors: Vec<Option<String>>,
    resolution: i32,
    number_of_dessin: Option<i32>,
    admin_password: String,
    password: Option<String>,
}

// #[derive(Serialize)]
// pub struct AddRoomRResponse {}

#[post("/add_room")]
async fn add_room(
    pool: web::Data<database::DbPool>,
    reqq_body: web::Json<AddRoomRequest>,
) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        let req_body = reqq_body.clone();
        let insert = InsertRoom {
            hashed_admin: hash(req_body.admin_password),
            hashed_password: req_body.password.map(|e| hash(e)),
            name: req_body.name,
            colors: req_body.colors,
            rules: req_body.rules,
            is_public: req_body.is_public,
            number_of_dessin: req_body.number_of_dessin,
            password_protected: req_body.password_protected,
            resolution: req_body.resolution,
        };
        database::add_room(&mut conn, &insert)
    })
    .await;
    match result {
        Err(e) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Err(e)) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Ok(room)) => HttpResponse::Ok().body(serde_json::to_string(&room).unwrap()),
    }
}

#[derive(Serialize)]
pub struct RequestResponse {
    pub message: String,
}

#[derive(Deserialize)]

struct DeleteRoomRequest {
    id: i32,
    password: String,
}

fn hash(pass: String) -> String {
    return pass;
}

#[post("/delete_room")]
async fn delete_room(
    pool: web::Data<database::DbPool>,
    req_body: web::Json<DeleteRoomRequest>,
) -> impl Responder {
    let result = web::block(move || {
        let mut conn: r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>> =
            pool.get().map_err(|e| e.to_string())?;
        let room = database::get_room(&mut conn, req_body.id)?;
        if room.hashed_admin != hash(req_body.password.clone()) {
            return Err("Wrong pass".to_string());
        }
        database::delete_room(&mut conn, req_body.id)
    })
    .await;

    match result {
        Err(e) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Err(e)) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Ok(())) => HttpResponse::Ok().body(
            serde_json::to_string(&RequestResponse {
                message: "room deleted".to_string(),
            })
            .unwrap(),
        ),
    }
}

// Cells
#[derive(Deserialize)]

struct DeleteCellRequest {
    room_id: i32,
    cell_id: i32,
    password: String,
}

#[post("/delete_cell")]
async fn delete_cell(
    pool: web::Data<database::DbPool>,
    req_body: web::Json<DeleteCellRequest>,
) -> impl Responder {
    let result = web::block(move || {
        let mut conn: r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>> =
            pool.get().map_err(|e| e.to_string())?;
        let room = database::get_room(&mut conn, req_body.room_id)?;
        if room.hashed_admin != hash(req_body.password.clone()) {
            return Err("Wrong pass".to_string());
        }
        database::delete_cell(&mut conn, req_body.cell_id)
    })
    .await;
    match result {
        Err(e) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Err(e)) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Ok(())) => HttpResponse::Ok().body(
            serde_json::to_string(&RequestResponse {
                message: "cell deleted".to_string(),
            })
            .unwrap(),
        ),
    }
}

#[derive(Deserialize)]

struct RequestDessinRequest {
    id: i32,
    password: Option<String>,
}

#[derive(Serialize)]
pub struct RequestDessinResponse {
    pub key: i32,
    pub side_cells: Vec<QueryDessin>,
    pub selected_cell: QueryDessin,
}

// fn select_force(available_cells: &Vec<QueryDessin>) -> Result<QueryDessin, &str> {
//     let mut distances = available_cells
//         .iter()
//         .map(|cell| cell.x * cell.x + cell.y * cell.y);
//     let min = distances.clone().min().ok_or("Empty available cells?")?;
//     let max = distances.clone().max().ok_or("Empty available cells?")?;

//     let posmin: usize = distances
//         .position(|elem| elem == min)
//         .ok_or("Min not found")?;

//     let min_cell = available_cells.get(posmin).ok_or("Min cell Not found")?;

//     if max - min > 33 {
//         return Ok(min_cell.to_owned());
//     }
//     return Err("No need");
// }

#[post("/request_dessin")]
async fn request_dessin(
    pool: web::Data<database::DbPool>,
    req_body: web::Json<RequestDessinRequest>,
) -> impl Responder {
    let result = web::block(move || {
        let mut conn: r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>> = pool
            .get()
            .map_err(|e| "could not get connection db ".to_string() + &e.to_string())?;

        let room = database::get_room(&mut conn, req_body.id)
            .map_err(|e| "Room not found : ".to_string() + &e)?;

        if room.password_protected {
            if !req_body
                .password
                .clone()
                .is_some_and(|value| Some(hash(value)) == room.hashed_password)
            {
                return Err("wrong pass".to_string());
            }
        }

        let selected_cell;

        let mut available_cells = get_available_cells(&mut conn, req_body.id.clone())
            .map_err(|e| "No available cells : ".to_string() + &e)?;

        if available_cells.is_empty() {
            selected_cell = create_first_cell(&mut conn, req_body.id.clone())
                .map_err(|e| "Empty available cells : ".to_string() + &e)?;
        } else {
            available_cells.sort_by(|a, b| {
                let c = b.x * b.x + b.y * b.y;
                (a.x * a.x + a.y * a.y).partial_cmp(&c).unwrap()
            });

            selected_cell = available_cells[0..available_cells.len() / 2]
                .choose(&mut rand::thread_rng())
                .ok_or_else(|| "no cell found".to_string())?
                .to_owned();
        }

        let side_cells = get_side_cells(
            &mut conn,
            selected_cell.x,
            selected_cell.y,
            selected_cell.room_id,
        )
        .map_err(|e| "could not get side cells : ".to_string() + &e)?;

        request_cell(&mut conn, &selected_cell)
            .map_err(|e| "could not request : ".to_string() + &e)?;

        Ok(RequestDessinResponse {
            key: selected_cell.id,
            side_cells,
            selected_cell: selected_cell,
        })
    })
    .await;

    match result {
        Err(e) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Err(e)) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Ok(cells)) => HttpResponse::Ok().body(serde_json::to_string(&cells).unwrap()),
    }
}

#[derive(Deserialize)]

struct CompleteDessinRequest {
    key: i32,
    id_room: i32,
    dessin: Vec<u8>,
    password: Option<String>,
}

#[derive(Serialize)]
pub struct CompleteDessinResponse {
    pub message: String,
}

#[post("/complete_dessin")]
async fn complete_dessin(
    pool: web::Data<database::DbPool>,
    req_body: web::Json<CompleteDessinRequest>,
) -> impl Responder {
    let result = web::block(move || {
        let mut conn: r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>> =
            pool.get().map_err(|e| e.to_string())?;

        let room = database::get_room(&mut conn, req_body.id_room)?;

        if room.password_protected {
            if !req_body
                .password
                .clone()
                .is_some_and(|value| Some(hash(value)) == room.hashed_password)
            {
                return Err("wrong pass".to_string());
            }
        }

        let cell = find_cell(&mut conn, req_body.key)?;

        check_cell(&cell, req_body.id_room)?;
        let _added: usize = add_adjacent_cell(&mut conn, &cell)?;

        validate_cell(&mut conn, req_body.dessin.clone(), &cell)?;

        Ok(CompleteDessinResponse {
            message: "dessin completed".to_string(),
        })
    })
    .await;

    match result {
        Err(e) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Err(e)) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Ok(cells)) => HttpResponse::Ok().body(serde_json::to_string(&cells).unwrap()),
    }
}

#[derive(Deserialize)]

struct GetCellsRequest {
    id: i32,
    password: Option<String>,
}

#[post("/get_dessins")]
async fn get_dessins(
    pool: web::Data<database::DbPool>,
    req_body: web::Json<GetCellsRequest>,
) -> impl Responder {
    let result = web::block(move || {
        let mut conn: r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>> =
            pool.get().map_err(|e| e.to_string())?;
        let room = database::get_room(&mut conn, req_body.id)?;

        if room.password_protected {
            if !req_body
                .password
                .clone()
                .is_some_and(|value| Some(hash(value)) == room.hashed_password)
            {
                return Err("wrong pass".to_string());
            }
        }

        database::get_dessins(&mut conn, req_body.id)
    })
    .await;
    match result {
        Err(e) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Err(e)) => HttpResponse::InternalServerError()
            .body(
                serde_json::to_string(&RequestResponse {
                    message: e.to_string(),
                })
                .unwrap(),
            )
            .into(),
        Ok(Ok(cells)) => HttpResponse::Ok().body(serde_json::to_string(&cells).unwrap()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Connecting to database...");
    dotenv().ok();
    // for (key, value) in env::vars() {
    //     println!("{key}: {value}");
    // }
    let url = env::var("DATABASE_URL").expect("no databse url");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Database not found");
    database::run_migrations(&pool);
    println!("Database connected.");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::permissive().allow_any_origin();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::new("%{r}a %U %s"))
            .service(hello)
            .service(get_rooms)
            .service(add_room)
            .service(delete_room)
            .service(get_dessins)
            .service(complete_dessin)
            .service(request_dessin)
            .service(delete_cell)
    })
    .bind(("0.0.0.0", 1420))?
    .run()
    .await
}
