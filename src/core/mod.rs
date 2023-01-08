mod visual;
pub struct Core {
    visual: visual::VisualEngine,
}

impl Core {
    pub fn init() -> Self {
        let core = Core {
            visual: visual::VisualEngine::init(),
        };

        return core;
    }
}
