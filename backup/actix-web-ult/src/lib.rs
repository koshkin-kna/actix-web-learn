extern crate actix_web;
extern crate tera;

pub mod middleware {
    #[cfg(debug_assertions)]
    use actix_web::middleware::{Middleware, Started};
    #[cfg(debug_assertions)]
    use actix_web::{HttpRequest, Result};
    use std::cell::RefCell;
    use tera;

    pub struct AppState {
        pub template: RefCell<tera::Tera>,
    }

    pub trait TemplateTera {
        fn get_tera(&self) -> &mut tera::Tera;
    }

    #[cfg(debug_assertions)]
    pub struct TemplateReload;

    #[cfg(debug_assertions)]
    impl<T: TemplateTera> Middleware<T> for TemplateReload {
        fn start(&self, req: &mut HttpRequest<T>) -> Result<Started> {
            //req.state().template.borrow_mut().full_reload().unwrap();
            //let test = req.state().get_tera().full_reload().unwrap();
            //test.full_reload().unwrap();
            req.state().get_tera().full_reload().unwrap();
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
