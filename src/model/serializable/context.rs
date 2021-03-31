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
pub struct BloodpressureListContext {
    pub app_context: AppContext,
    pub bloodpressures: Vec<SerBloodPressure>
}
