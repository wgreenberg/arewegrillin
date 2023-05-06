use std::sync::RwLock;
use std::env::var;
use rocket::{
    launch,
    post,
    get,
    routes,
    State,
    FromForm,
    form::Form,
};
use rocket_dyn_templates::{Template, context};

type Status = RwLock<String>;

#[get("/")]
fn index(status: &State<Status>) -> Template {
    Template::render("index", context! {
        status: status.read().unwrap().to_string()
    })
}

#[derive(FromForm)]
struct SetRequest {
    status: String,
    password: String,
}

#[get("/set")]
fn set_form(status: &State<Status>) -> Template {
    Template::render("set", context! {
        status: status.read().unwrap().to_string()
    })
}

#[post("/set", data = "<request>")]
fn set(status: &State<Status>, request: Form<SetRequest>) -> &'static str {
    let expected_password = var("SECRET").unwrap_or("grillpilled1312".into());
    if request.password != expected_password {
        "never talk to me again"
    } else {
        *status.write().unwrap() = request.status.clone();
        "ok"
    }
}

#[launch]
fn serve() -> _ {
    let status = "NO".to_string();
    rocket::build()
        .mount("/", routes![index, set_form, set])
        .manage(RwLock::new(status))
        .attach(Template::fairing())
}
