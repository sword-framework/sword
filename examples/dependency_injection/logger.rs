use shaku::{Component, Interface};

pub trait Logger: Interface {
    fn log(&self, message: &str);
}

#[derive(Component)]
#[shaku(interface = Logger)]
pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("Log: {}", message);
    }
}
