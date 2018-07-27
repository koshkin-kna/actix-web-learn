use actix_web::http::{ContentEncoding};
use actix_web::{pred, App, HttpRequest, HttpResponse};
use std::cell::RefCell;
use tera;
use normalize_path::NormalizePathCustom;
use actix_web_ult::tmp_engine::TemplateEngine;

// If debug true
#[cfg(debug_assertions)] 
use actix_web_ult::middleware::MiddlewareTemplateEngineReload;
#[cfg(debug_assertions)]
use actix_web::fs;
#[cfg(debug_assertions)] 
use actix_web::{middleware};


pub struct AppState {
    pub template: RefCell<tera::Tera>,
}

impl TemplateEngine for AppState {
    fn template_reload(&self) {
        self.template.borrow_mut().full_reload().unwrap();
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

    // If debug true
    #[cfg(debug_assertions)]
    let app = app.middleware(middleware::Logger::default())
    .middleware(MiddlewareTemplateEngineReload)
    .handler("/static", fs::StaticFiles::new("./src/static/build").show_files_listing(),);
    

    app.resource("/", |r| r.f(index))
        .resource("/{test}/", |r| r.f(index))
        .default_resource(|r| {
            r.h(NormalizePathCustom::default());
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|_req| HttpResponse::MethodNotAllowed());
        })
}
