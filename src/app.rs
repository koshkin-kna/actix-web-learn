use actix_web::{pred, App, HttpResponse};
use std::cell::RefCell;
use tera;
use normalize_path::NormalizePathCustom;
use actix_web_ult::tmp_engine::TemplateEngine;
use urls::urls_pattern;

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

/// Функция создаёт Application
pub fn create_app() -> App<AppState> {
    let app = App::with_state(AppState {
        template: RefCell::new(compile_templates!("./src/templates/**/*")),
    });

    #[cfg(debug_assertions)]
    let app = app.middleware(middleware::Logger::default());
    #[cfg(debug_assertions)]
    let app = app.middleware(MiddlewareTemplateEngineReload);
    
    let app = urls_pattern(app);

    #[cfg(debug_assertions)]
    let app = app.handler("/static", fs::StaticFiles::new("./src/static/build").show_files_listing(),);
    
    app.default_resource(|r| {
            r.h(NormalizePathCustom::default());
            r.route()
                .filter(pred::Not(pred::Get()))
                .f(|_req| HttpResponse::MethodNotAllowed());
        })
}
