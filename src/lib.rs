use visual_engine::VisualEngine;

pub struct PsychedelicEngine {
    visual_engine: VisualEngine,
}

impl PsychedelicEngine {
    pub fn init() -> Self {
        let visual_engine = VisualEngine::init();

        let pyschedelic_engine = Self {
            visual_engine: visual_engine,
        };

        return pyschedelic_engine;
    }

    pub fn start(self) {
        self.visual_engine.start();
    }
}
