use std::time::SystemTime;

use gourmand_zombie::{
    models::{InsertNewDessin, InsertRoom, QueryDessin, QueryRoom, RequestDessin, ValidateDessin},
    schema::{dessins, rooms},
};
use diesel::{r2d2, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn run_migrations(pool: &DbPool) {
    let mut conn = pool.get().expect("Impossible de récupérer une connexion pour les migrations");
    conn.run_pending_migrations(MIGRATIONS).expect("Échec lors de l'exécution des migrations Diesel");
    println!("✅ Migrations Diesel vérifiées et appliquées avec succès !");
}

pub fn create_first_cell(conn: &mut PgConnection, room_id: i32) -> Result<QueryDessin, String> {
    let id: i32 = diesel::insert_into(dessins::table)
        .values(InsertNewDessin {
            done: false,
            room_id: room_id,
            x: 0,
            y: 0,
        })
        .on_conflict((dessins::x, dessins::y, dessins::room_id))
        .do_nothing()
        .returning(dessins::id)
        .get_result(conn)
        .map_err(|e| e.to_string())?;

    let dessin = QueryDessin {
        id,
        room_id,
        x: 0,
        y: 0,
        content: None,
        done: false,
        last_request: None,
    };
    return Ok(dessin);
}

pub fn get_dessins(conn: &mut PgConnection, room_id: i32) -> Result<Vec<QueryDessin>, String> {
    dessins::table
        // .filter(dessins::done.eq(true))
        .filter(dessins::room_id.eq(room_id))
        .load::<QueryDessin>(conn)
        .map_err(|err| err.to_string())
}

pub fn get_available_cells(
    conn: &mut PgConnection,
    id_room: i32,
) -> Result<Vec<QueryDessin>, String> {
    let available_cells = dessins::table
        .filter(dessins::room_id.eq(id_room))
        .filter(dessins::done.eq(false))
        // .filter(dessins::last_request.is_null())
        //.filter(dessins::last_request.lt(min_time))
        .load::<QueryDessin>(conn)
        .map_err(|e| e.to_string())?;

    // let min_time = SystemTime::now()
    //     .checked_sub(Duration::from_secs(600))
    //     .map_or(Err("Erreur du temps"), |r| Ok(r))?;

    // let filtered: Vec<QueryDessin> = available_cells
    //     .into_iter()
    //     .filter(|cell| {
    //         if let Some(last_time) = cell.last_request {
    //             if last_time < min_time {
    //                 return true;
    //             } else {
    //                 // remove those with a last_time used after 10 minute ago
    //                 return false;
    //             }
    //         } else {
    //             return true;
    //         }
    //     })
    //     .collect();

    return Ok(available_cells);
}

pub fn get_side_cells(
    conn: &mut PgConnection,
    x: i32,
    y: i32,
    id_room: i32,
) -> Result<Vec<QueryDessin>, String> {
    dessins::table
        .filter(dessins::room_id.eq(id_room))
        .filter(dessins::x.between(x - 1, x + 1))
        .filter(dessins::y.between(y - 1, y + 1))
        .load::<QueryDessin>(conn)
        .map_err(|e| e.to_string())
}

pub fn find_cell(conn: &mut PgConnection, id: i32) -> Result<QueryDessin, String> {
    dessins::table
        .filter(dessins::id.eq(id))
        .first::<QueryDessin>(conn)
        .map_err(|err| err.to_string())
}

pub fn check_cell(cell: &QueryDessin, room_id: i32) -> Result<(), String> {
    if cell.room_id != room_id {
        return Err("cell in another room".to_string());
    }
    if cell.done {
        return Err("cell already done".to_string());
    }
    // let min_time = SystemTime::now()
    //     .checked_sub(Duration::from_secs(600))
    //     .unwrap(); // - 10 minutes
    // if cell.last_request.is_none() {
    //     return Ok(());
    // }
    // no need for now
    // if cell.last_request.unwrap() < min_time {
    //     return Err("cell too late".to_string());
    // }
    return Ok(());
}

pub fn validate_cell(
    conn: &mut PgConnection,
    mdessin: Vec<u8>,
    cell: &QueryDessin,
) -> Result<usize, String> {
    let r = ValidateDessin {
        content: Some(mdessin),
        done: true,
    };
    diesel::update(cell)
        .set(&r)
        .execute(conn)
        .map_err(|err| err.to_string())
}

pub fn add_adjacent_cell(conn: &mut PgConnection, cell: &QueryDessin) -> Result<usize, String> {
    let mut v: Vec<InsertNewDessin> = vec![];
    v.push(InsertNewDessin {
        done: false,
        room_id: cell.room_id,
        x: cell.x + 1,
        y: cell.y,
    });
    v.push(InsertNewDessin {
        done: false,
        room_id: cell.room_id,
        x: cell.x - 1,
        y: cell.y,
    });
    v.push(InsertNewDessin {
        done: false,
        room_id: cell.room_id,
        x: cell.x,
        y: cell.y + 1,
    });
    v.push(InsertNewDessin {
        done: false,
        room_id: cell.room_id,
        x: cell.x,
        y: cell.y - 1,
    });

    diesel::insert_into(dessins::table)
        .values(&v)
        .on_conflict((dessins::x, dessins::y, dessins::room_id))
        .do_nothing()
        .execute(conn)
        .map_err(|e| e.to_string())
}

// //todo generate secret
pub fn request_cell(conn: &mut PgConnection, cell: &QueryDessin) -> Result<(), String> {
    let r = RequestDessin {
        last_request: Some(SystemTime::now()),
    };

    let nb: usize = diesel::update(cell)
        .set(&r)
        .execute(conn)
        .map_err(|err| err.to_string())?;
    if nb != 1 {
        return Err("cound not request cell".to_string());
    }
    return Ok(());
}

// rooms

pub fn get_rooms(conn: &mut PgConnection) -> Result<Vec<QueryRoom>, String> {
    rooms::table
        .filter(rooms::is_public.eq(true))
        .load::<QueryRoom>(conn)
        .map_err(|err| err.to_string() + " (get_rooms_error)")
}

pub fn get_room(conn: &mut PgConnection, id: i32) -> Result<QueryRoom, String> {
    rooms::table
        .filter(rooms::id.eq(id))
        .first::<QueryRoom>(conn)
        .map_err(|err| err.to_string() + " (get_room_error)")
}

pub fn delete_room(conn: &mut PgConnection, id: i32) -> Result<(), String> {
    let deleted = diesel::delete(rooms::table)
        .filter(rooms::id.eq(id))
        .execute(conn)
        .map_err(|err| err.to_string() + " (get_room_error)")?;

    if deleted != 1 {
        return Err("could not delete".to_string());
    }
    Ok(())
}

pub fn delete_cell(conn: &mut PgConnection, id: i32) -> Result<(), String> {
    let deleted = diesel::delete(dessins::table)
        .filter(dessins::id.eq(id))
        .execute(conn)
        .map_err(|err| err.to_string() + " (delete_cell_error)")?;

    if deleted != 1 {
        return Err("could not delete".to_string());
    }
    Ok(())
}

pub fn add_room(conn: &mut PgConnection, room: &InsertRoom) -> Result<QueryRoom, String> {
    let inserted: QueryRoom = diesel::insert_into(rooms::table)
        .values(room)
        .get_result(conn)
        .map_err(|e| e.to_string())?;

    Ok(inserted)
}
