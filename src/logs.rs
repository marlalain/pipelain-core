use crate::World;

pub struct Log {
    pub entries: Vec<String>,
}

impl Log {
    pub fn info<T: ToString>(world: &World, message: T) {
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
