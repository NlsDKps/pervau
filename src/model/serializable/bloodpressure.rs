use rocket::FromForm;
use serde_derive::{Deserialize, Serialize};
use crate::{
    model::database::bloodpressure::BloodPressure,
};

/// Struct to serialize for further usage.
#[derive(Serialize)]
pub struct SerBloodPressure {
    /// Id of the entity in the database
    pub id : i32,
    /// User id associated to the bloodpressure entity
    pub user_id : i32,
    /// Date when the bloodpressure was measured
    /// # Note
    /// It is not possible to serialize NaiveDate with SerDe
    pub date : String,
    /// Time when the bloodpressure was measured
    /// # Note
    /// It is not possible to serialize NaiveTime with SerDe
    pub time : String,
    /// The systole
    pub systole : i32,
    /// The diastole
    pub diastole : i32,
    /// The pulse
    pub pulse : i32
}

impl SerBloodPressure {
    /// Constructor
    ///
    /// Constructs new object from Bloodpressure object
    pub fn new(bp: &BloodPressure) -> Self {
        SerBloodPressure {
            id: bp.id,
            user_id: bp.user_id,
            date: bp.date.clone(),
            time: bp.time.clone(),
            systole: bp.systole,
            diastole: bp.diastole,
            pulse: bp.pulse
        }
    }
}

/// Struct to deserialize from either an JSON-structure or a form.
#[derive(FromForm, Deserialize)]
pub struct NewSerBloodPressure {
    /// Date when the bloodpressure was measured
    /// # Note
    /// It is not possible to serialize NaiveDate with SerDe
    pub date : String,
    /// Time when the bloodpressure was measured
    /// # Note
    /// It is not possible to serialize NaiveTime with SerDe
    pub time : String,
    /// The systole
    pub systole : i32,
    /// The diastole
    pub diastole : i32,
    /// The pulse
    pub pulse : i32
}

