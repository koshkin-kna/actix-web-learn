extern crate actix_web;

pub mod middleware {
    use actix_web::middleware::{Middleware, Started};
    use actix_web::{HttpRequest, Result};

    pub trait TemplateEngine {
        fn template_reload(&self);
    }

    pub struct MiddlewareTemplateEngineReload;

    impl<T: TemplateEngine> Middleware<T> for MiddlewareTemplateEngineReload {
        fn start(&self, req: &mut HttpRequest<T>) -> Result<Started> {
            req.state().template_reload();
            Ok(Started::Done)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
