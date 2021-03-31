use rocket::{
    response::Redirect,
    request::Form,
    State
};
use rocket_contrib::templates::Template;
use crate::{
    controller::management::bloodpressure as mgmt_bp,
    model::serializable::{
        bloodpressure::NewSerBloodPressure,
        context::{
            AppContext,
            BloodpressureListContext
        }
    },
};

#[get("/")]
pub fn list(
    db_url: State<String>
) -> Template {
    let bps = mgmt_bp::read_all(&db_url);
    let context = BloodpressureListContext {
        app_context : AppContext::new(),
        bloodpressures: bps
    };
    Template::render("bloodpressure/list", &context)
}

#[post("/add", data="<bp_info>")]
pub fn add(
    db_url: State<String>,
    bp_info: Form<NewSerBloodPressure>
) -> Redirect {
    mgmt_bp::add(&db_url, &bp_info);
    Redirect::to("/")
}

