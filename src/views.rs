use actix_web::{HttpRequest, HttpResponse};
use app::AppState;
use actix_web::http::{ContentEncoding};
use tera;

pub fn index(req: HttpRequest<AppState>) -> HttpResponse {
    let template = req.state().template.borrow();
    let mut context = tera::Context::new();
    context.add("vat_rate", &0.20);
    let result = template.render("admin/login.html", &context).unwrap();
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html; charset=utf-8")
        .body(result)
}