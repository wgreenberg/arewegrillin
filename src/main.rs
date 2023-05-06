use std::sync::Mutex;
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

type Status = Mutex<String>;

#[get("/")]
fn index(status: &State<Status>) -> Template {
    Template::render("index", context! {
        status: status.lock().unwrap().to_string()
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
        status: status.lock().unwrap().to_string()
    })
}

#[post("/set", data = "<request>")]
fn set(status: &State<Status>, request: Form<SetRequest>) -> &'static str {
    let expected_password = var("SECRET").unwrap_or("grillpilled".into());
    if request.password != expected_password {
        "never talk to me again"
    } else {
        *status.lock().unwrap() = request.status.clone();
        "ok"
    }
}

#[launch]
fn serve() -> _ {
    let status = "NO".to_string();
    rocket::build()
        .mount("/", routes![index, set_form, set])
        .manage(Mutex::new(status))
        .attach(Template::fairing())
}
