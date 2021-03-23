#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::*;

fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("./mydb.db").expect("Database file not found")
}

fn insert_fake_entries() {
    use schema::Projects::dsl::*;
    let conn = establish_connection();

    let new_project = NewProject {
        proj_id: Some(1),
        proj_title: "test project".to_string(),
        proj_cost: 30,
        proj_duration: Some(100)
    };
    insert_into(Projects).values(&new_project).execute(&conn);

    let result = Projects.load::<Project>(&conn).expect("Error loading Projects");
    println!("-----------------");
    for proj in result {
        println!("{:?}", proj.proj_id);
    }
    println!("-----------------");
}

#[cfg(test)]
mod tests {

    use crate::*;
    #[test]
    fn test_insert_entries() {
        // assert_eq!(2 + 2, 4);
        insert_fake_entries();
    }
}
