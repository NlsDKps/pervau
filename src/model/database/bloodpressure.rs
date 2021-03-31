use crate::{
    model::serializable::bloodpressure::{NewSerBloodPressure},
    schema::bloodpressures,
};

/// Queryable struct representing an bloodpressure entity
#[derive(Queryable)]
pub struct BloodPressure {
    /// Id of the entity in the database
    pub id : i32,
    /// User id associated to the bloodpressure entity
    pub user_id : i32,
    /// Date when the bloodpressure was measured
    /// # Note
    /// The datatype date is not available in sqlite
    pub date : String,
    /// Time when the bloodpressure was measured
    /// # Note
    /// The datatype time is not available in sqlite
    pub time : String,
    /// The systole
    pub systole : i32,
    /// The diastole
    pub diastole : i32,
    /// The pulse
    pub pulse : i32
}

/// Insertable struct representing an bloodpressure entity.
#[derive(Insertable)]
#[table_name="bloodpressures"]
pub struct NewBloodPressure {
    /// User id associated to the bloodpressure entity
    pub user_id : i32,
    /// Date when the bloodpressure was measured
    /// # Note
    /// The datatype date is not available in sqlite
    pub date : String,
    /// Time when the bloodpressure was measured
    /// # Note
    /// The datatype time is not available in sqlite
    pub time : String,
    /// The systole
    pub systole : i32,
    /// The diastole
    pub diastole : i32,
    /// The pulse
    pub pulse : i32
}

impl NewBloodPressure {
    pub fn new(new_ser_bp: &NewSerBloodPressure) -> Self {
        NewBloodPressure {
            user_id: 0,
            date: new_ser_bp.date.clone(),
            time: new_ser_bp.time.clone(),
            systole: new_ser_bp.systole,
            diastole: new_ser_bp.diastole,
            pulse: new_ser_bp.pulse
        }
    }
}
