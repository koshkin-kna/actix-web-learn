use actix_web::http::{header, StatusCode};
use regex::Regex;
use actix_web::{HttpRequest, HttpResponse, dev::Handler};
use tmp_engine::TemplateEngine;
use tera;
use actix_web::http::{ContentEncoding};


pub struct NormalizePath {
    append: bool,
    merge: bool,
    re_merge: Regex,
    redirect: StatusCode,
    not_found: StatusCode,
}

impl Default for NormalizePath {
    /// Create default `NormalizePath` instance, *append* is set to *true*,
    /// *merge* is set to *true* and *redirect* is set to
    /// `StatusCode::MOVED_PERMANENTLY`
    fn default() -> NormalizePath {
        NormalizePath {
            append: true,
            merge: true,
            re_merge: Regex::new("//+").unwrap(),
            redirect: StatusCode::MOVED_PERMANENTLY,
            not_found: StatusCode::NOT_FOUND,
        }
    }
}

impl NormalizePath {
    /// Create new `NormalizePath` instance
    pub fn new(append: bool, merge: bool, redirect: StatusCode) -> NormalizePath {
        NormalizePath {
            append,
            merge,
            redirect,
            re_merge: Regex::new("//+").unwrap(),
            not_found: StatusCode::NOT_FOUND,
        }
    }
}

impl<T: TemplateEngine> Handler<T> for NormalizePath {
    type Result = HttpResponse;

    fn handle(&mut self, req: HttpRequest<T>) -> Self::Result {
        if let Some(router) = req.router() {
            let query = req.query_string();
            if self.merge {
                // merge slashes
                let p = self.re_merge.replace_all(req.path(), "/");
                if p.len() != req.path().len() {
                    if router.has_route(p.as_ref()) {
                        let p = if !query.is_empty() {
                            p + "?" + query
                        } else {
                            p
                        };
                        return HttpResponse::build(self.redirect)
                            .header(header::LOCATION, p.as_ref())
                            .finish();
                    }
                    // merge slashes and append trailing slash
                    if self.append && !p.ends_with('/') {
                        let p = p.as_ref().to_owned() + "/";
                        if router.has_route(&p) {
                            let p = if !query.is_empty() {
                                p + "?" + query
                            } else {
                                p
                            };
                            return HttpResponse::build(self.redirect)
                                .header(header::LOCATION, p.as_str())
                                .finish();
                        }
                    }

                    // try to remove trailing slash
                    if p.ends_with('/') {
                        let p = p.as_ref().trim_right_matches('/');
                        if router.has_route(p) {
                            let mut req = HttpResponse::build(self.redirect);
                            return if !query.is_empty() {
                                req.header(
                                    header::LOCATION,
                                    (p.to_owned() + "?" + query).as_str(),
                                )
                            } else {
                                req.header(header::LOCATION, p)
                            }.finish();
                        }
                    }
                } else if p.ends_with('/') {
                    // try to remove trailing slash
                    let p = p.as_ref().trim_right_matches('/');
                    if router.has_route(p) {
                        let mut req = HttpResponse::build(self.redirect);
                        return if !query.is_empty() {
                            req.header(
                                header::LOCATION,
                                (p.to_owned() + "?" + query).as_str(),
                            )
                        } else {
                            req.header(header::LOCATION, p)
                        }.finish();
                    }
                }
            }
            // append trailing slash
            if self.append && !req.path().ends_with('/') {
                let p = req.path().to_owned() + "/";
                if router.has_route(&p) {
                    let p = if !query.is_empty() {
                        p + "?" + query
                    } else {
                        p
                    };
                    return HttpResponse::build(self.redirect)
                        .header(header::LOCATION, p.as_str())
                        .finish();
                }
            }
        }
        let template_engine = req.state().get_engine();
        let context = tera::Context::new();
        let result = template_engine.render("404.html", &context).unwrap();
        HttpResponse::build(self.not_found)
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html")
        .body(result)
    }
}
