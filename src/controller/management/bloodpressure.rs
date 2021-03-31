use crate::{
    controller::database::bloodpressure as db_bp,
    model::{
        serializable::bloodpressure::{NewSerBloodPressure, SerBloodPressure},
        database::bloodpressure::{NewBloodPressure},
    },
};
pub fn add<'a>(
    db_url: &'a str,
    new_ser_bp: &'a NewSerBloodPressure
) -> bool {
    let new_bp = NewBloodPressure::new(new_ser_bp);
    match db_bp::create(db_url, &new_bp) {
        Some(_) => true,
        None => false
    }
}

pub fn read_by_id<'a>(
    db_url: &'a str,
    id: &'a i32
) -> Option<SerBloodPressure> {
    match db_bp::read_by_id(db_url, id) {
        Some(bp) => Some(SerBloodPressure::new(&bp)),
        None => None
    }
}

pub fn read_all<'a>(
    db_url: &'a str
) -> Vec<SerBloodPressure> {
    match db_bp::read_all(db_url) {
        Some(entities) => {
            let mut ser_entities = vec!();
            for entity in entities {
                let ser_entity = SerBloodPressure::new(&entity);
                ser_entities.push(ser_entity);
            }
            ser_entities
        },
        None => vec!()
    }
}

pub fn delete<'a>(
    db_url: &'a str,
    id: &'a i32
) -> bool {
    db_bp::delete(db_url, id)
}
