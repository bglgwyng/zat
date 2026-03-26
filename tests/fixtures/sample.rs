use std::collections::HashMap;

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn helper() -> i32 {
    42
}

pub struct Config {
    pub name: String,
    pub value: i32,
    secret: String,
}

pub enum Color {
    Red,
    Green,
    Blue(u8, u8, u8),
}

pub trait Drawable {
    fn draw(&self);
    fn visible(&self) -> bool;
}

pub impl Config {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: 0,
            secret: String::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn internal(&self) {}
}

pub type Mapping = HashMap<String, String>;

pub const MAX_SIZE: usize = 1024;

pub static GLOBAL: &str = "hello";

pub mod utils {
    pub fn parse(input: &str) -> Result<(), String> {
        Ok(())
    }
}
