/*use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{header, StatusCode, ContentEncoding};
use app::AppState;
use tera;

struct Http404;

impl Http404 {
    pub fn new(req: HttpRequest<AppState>) -> HttpResponse {
        let template = req.state().template.borrow();
        let context = tera::Context::new();
        let result = template.render("404.html", &context).unwrap();
        HttpResponse::build(StatusCode::NOT_FOUND)
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html")
        .body(result)
    }
}*/
