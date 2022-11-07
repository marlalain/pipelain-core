use std::ops::{Deref, DerefMut};

use crate::World;

pub struct Log {
    pub entries: Vec<String>,
}

impl Log {
    pub fn log<T: ToString>(&mut self, message: T) {
        let last_log = self
            .entries
            .get(self.entries.len() - 1)
            .expect("out of bounds");

        if last_log != &message.to_string() {
            self.entries.push(message.to_string())
        }
    }

    pub fn by_world<T: ToString>(world: &World, message: T) {
        let mut log = world.fetch_mut::<Log>();

        let last_log = log
            .entries
            .get(log.entries.len() - 1)
            .expect("out of bounds");

        if last_log != &message.to_string() {
            log.entries.push(message.to_string())
        }
    }
}
