//! Дополнительнеы middleware упрощающие разработку приложения

use actix_web::middleware::{Middleware, Started};
use tmp_engine::TemplateEngine;
use actix_web::{HttpRequest, Result};

/// Middleware для dev режима, для каждого запроса перезагружает шаблоны
pub struct MiddlewareTemplateEngineReload;

impl<T: TemplateEngine> Middleware<T> for MiddlewareTemplateEngineReload {
    /// Реализация типажа Middleware (start), то есть перезагружает шаблон на входе request
    fn start(&self, req: &mut HttpRequest<T>) -> Result<Started> {
        let _ = req.state().get_engine_mut().full_reload();
        Ok(Started::Done)
    }
}