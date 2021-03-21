table! {
    Employee_projects (emp_id, proj_id) {
        emp_id -> Nullable<Integer>,
        proj_id -> Nullable<Integer>,
    }
}

table! {
    Employees (emp_id) {
        emp_id -> Nullable<Integer>,
        emp_name -> Text,
        emp_desig -> Nullable<Text>,
        emp_salary -> Nullable<Text>,
    }
}

table! {
    Projects (proj_id) {
        proj_id -> Nullable<Integer>,
        proj_title -> Text,
        proj_cost -> Integer,
        proj_duration -> Nullable<Integer>,
    }
}

joinable!(Employee_projects -> Employees (emp_id));
joinable!(Employee_projects -> Projects (proj_id));

allow_tables_to_appear_in_same_query!(
    Employee_projects,
    Employees,
    Projects,
);
