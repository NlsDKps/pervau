use log::error;
use diesel::prelude::*;

use crate::{
    controller::database::connect_database,
    model::database::bloodpressure::{NewBloodPressure, BloodPressure}
};

/// Create new bloodpressure entry
pub fn create<'a>(
    db_url: &'a str,
    new_bp: &'a NewBloodPressure
) -> Option<usize> {
    use crate::schema::bloodpressures;
    match connect_database(&db_url) {
        Some(conn) => {
            match diesel::insert_into(bloodpressures::table)
                .values(new_bp)
                .execute(&conn)
            {
                Ok(no_added) => Some(no_added),
                Err(e) => {
                    error!("Could not create new bloodpressure: {}", e);
                    None
                }
            }
        },
        None => None
    }
}

/// Read all bloodpressure entries
pub fn read_all<'a>(
    db_url: &'a str
) -> Option<Vec<BloodPressure>> {
    use crate::schema::bloodpressures::dsl::*;
    match connect_database(&db_url) {
        Some(conn) => {
            match bloodpressures.load::<BloodPressure>(&conn) {
                Ok(entities) => Some(entities),
                Err(e) => {
                    error!("Could not fetch bloodpressure: {}", e);
                    None
                }
            }
        },
        None => None
    }
}

/// Read a bloodpressure entry by its id
pub fn read_by_id<'a>(
    db_url: &'a str,
    bp_id: &'a i32
) -> Option<BloodPressure> {
    use crate::schema::bloodpressures::dsl::*;
    match connect_database(&db_url) {
        Some(conn) => {
            match bloodpressures.filter(id.eq(bp_id)).load::<BloodPressure>(&conn) {
                Ok(mut entities) => {
                    if entities.len() == 0 {
                        None
                    } else {
                        let first = entities.remove(0);
                        Some(first)
                    }
                },
                Err(e) => {
                    error!("Could not fetch bloodpressure: {}", e);
                    None
                }
            }
        },
        None => None
    }
}

/// Delete a bloodpressure entry by its id
pub fn delete<'a>(
    db_url: &'a str,
    bp_id: &'a i32
) -> bool {
    use crate::schema::bloodpressures::dsl::*;
    match connect_database(&db_url) {
        Some(conn) => {
            match diesel::delete(bloodpressures.filter(id.eq(bp_id))).execute(&conn) {
                Ok(_) => true,
                Err(e) => {
                    error!("Could not delete bloodpressure: {}", e);
                    false
                }
            }
        },
        None => false
    }
}
