mod core;

pub struct Engine {
    core: core::Core,
}

impl Engine {
    pub fn init() -> Self {
        let engine = Self {
            core: core::Core::init(),
        };

        return engine;
    }
}
