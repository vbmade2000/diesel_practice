use diesel::prelude::*;
use super::schema::*;

#[derive(Queryable)]
pub struct Project {
    pub proj_id: Option<i32>,
    pub proj_title: String,
    pub proj_cost: i32,
    pub proj_duration: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "Projects"]
pub struct NewProject {
    pub proj_id: Option<i32>,
    pub proj_title: String,
    pub proj_cost: i32,
    pub proj_duration: Option<i32>,
}
