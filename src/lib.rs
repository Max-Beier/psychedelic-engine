use visual_engine::VisualEngine;

pub use psychedelic_types::{engine, Mesh3D, Object3D};

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

    pub fn load_mesh(&mut self, mesh: Mesh3D) {
        self.visual_engine.load_mesh(mesh);
    }

    pub fn start(self) {
        self.visual_engine.start();
    }
}
