use super::schema::{habitmetric, habitname};

#[derive(Insertable, Queryable)]
#[table_name = "habitname"]
pub struct NewHabit<'a> {
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "habitmetric"]
pub struct NewMetric<'a> {
    pub habitname_id: &'a i32,
    pub units: &'a i32,
}

#[derive(Queryable)]
pub struct Metric<'a> {
    pub habitname_id: &'a i32,
    pub units: &'a i32,
}
