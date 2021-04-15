use chrono::Utc;
use log::debug;
use crate::model::serializable::bloodpressure::SerBloodPressure;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct AppContext {
    pub app_name: String
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            app_name: String::from("PerVau")
        }
    }
}

#[derive(Serialize)]
pub struct AddBloodpressurePresetsContext {
    pub current_date: String,
    pub current_time: String
}

impl AddBloodpressurePresetsContext {
    pub fn new() -> Self {
        let cdatetime = Utc::now();
        let cdate = cdatetime.date().naive_utc().to_string();
        let ctime = cdatetime.time().format("%H:%M").to_string();
        debug!("ctime: {}", ctime);
        AddBloodpressurePresetsContext {
            current_date: cdate,
            current_time: ctime
        }
    }
}

#[derive(Serialize)]
pub struct BloodpressureListContext {
    pub app_context: AppContext,
    pub preset_context: AddBloodpressurePresetsContext,
    pub bloodpressures: Vec<SerBloodPressure>
}
