use crate::World;

pub struct Log {
    pub entries: Vec<String>,
}

impl Log {
    pub fn info(world: &World, message: &str) {
        Log::_info(world, message.to_string())
    }

    fn _info(world: &World, message: String) {
        let mut log = world.fetch_mut::<Log>();
        let last_log = log
            .entries
            .get(log.entries.len() - 1)
            .expect("out of bounds");

        if last_log != &message {
            log.entries.push(message)
        }
    }
}
