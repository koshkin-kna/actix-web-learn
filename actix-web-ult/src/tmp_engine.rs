
//! Определяет типаж необходимый для работы с шаблонами (Tera)

use std::cell::RefMut;
use std::cell::Ref;
use tera;

pub trait TemplateEngine {
    fn get_engine_mut(&self) -> RefMut<tera::Tera>;
    fn get_engine(&self) -> Ref<tera::Tera>;
}