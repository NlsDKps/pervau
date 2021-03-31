use rocket::State;
use rocket_contrib::json::Json;
use crate::{
    controller::management::bloodpressure as mgmt_bp,
    model::serializable::bloodpressure::{NewSerBloodPressure, SerBloodPressure},
};

#[post("/bloodpressure/add", format="application/json", data="<create_info>")]
pub fn add(
    db_url: State<String>, create_info: Json<NewSerBloodPressure>
) -> Json<bool> {
    Json(mgmt_bp::add(&db_url, &create_info))
}

#[get("/bloodpressure/all")]
pub fn get_all(
    db_url: State<String>
) -> Json<Vec<SerBloodPressure>> {
    Json(mgmt_bp::read_all(&db_url))
}

#[get("/bloodpressure/<id>")]
pub fn get_by_id(
    db_url: State<String>,
    id: i32
) -> Json<Option<SerBloodPressure>> {
    Json(mgmt_bp::read_by_id(&db_url, &id))
}

#[delete("/bloodpressure/<id>")]
pub fn delete_by_id(
    db_url: State<String>,
    id: i32
) -> Json<bool> {
    Json(mgmt_bp::delete(&db_url, &id))
}
