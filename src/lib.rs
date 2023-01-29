use psychedelic_types::{engine::Vertex3D, Mesh3D};
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

    pub fn start(mut self) {
        let mut mesh = Mesh3D::new();
        mesh.vertices = vec![
            Vertex3D {
                position: [-0.5, -0.5],
            },
            Vertex3D {
                position: [0.0, 0.5],
            },
            Vertex3D {
                position: [0.25, -0.1],
            },
        ];

        self.visual_engine.load_mesh(mesh);

        self.visual_engine.start();
    }
}
