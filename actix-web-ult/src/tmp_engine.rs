//! Определяет типаж необходимый для работы с шаблонами (Tera)

pub trait TemplateEngine {
    /// Метод перезагрузки шаблонов из директории
    fn template_reload(&self);
}