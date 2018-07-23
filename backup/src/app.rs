use actix_web::http::{ContentEncoding};
#[cfg(debug_assertions)] 
use actix_web::middleware::{Middleware, Started};
use actix_web::{fs, pred, App, HttpRequest, HttpResponse};
#[cfg(debug_assertions)] 
use actix_web::{middleware, Result};
use std::cell::RefCell;
use tera;
use normalize_path::NormalizePathCustom;


pub struct AppState {
    pub template: RefCell<tera::Tera>,
}


#[cfg(debug_assertions)]
struct TemplateReload;

#[cfg(debug_assertions)]
impl Middleware<AppState> for TemplateReload {
    fn start(&self, req: &mut HttpRequest<AppState>) -> Result<Started> {
        req.state().template.borrow_mut().full_reload().unwrap();
        Ok(Started::Done)
    }
}


fn index(req: HttpRequest<AppState>) -> HttpResponse {
    let template = req.state().template.borrow();
    let mut context = tera::Context::new();
    context.add("vat_rate", &0.20);
    let result = template.render("admin/login.html", &context).unwrap();
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html; charset=utf-8")
        .body(result)
}


pub fn create_app() -> App<AppState> {
    let app = App::with_state(AppState {
        template: RefCell::new(compile_templates!("./src/templates/**/*")),
    });
    #[cfg(debug_assertions)]
    let app = app.middleware(middleware::Logger::default());
    #[cfg(debug_assertions)]
    let app = app.middleware(TemplateReload);
    app.resource("/", |r| r.f(index))
        .resource("/{test}/", |r| r.f(index))
        .handler(
            "/static",
            fs::StaticFiles::new("./src/static/build").show_files_listing(),
        )
        .default_resource(|r| {
            r.h(NormalizePathCustom::default());
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|_req| HttpResponse::MethodNotAllowed());
        })
}
