# Psychedelic Engine
Lightweight game engine written in Rust.

# UPDATE: Project currently on hold due to commercial projects

## Getting Started
```rust
use psychedelic_engine as pe;

fn main() {
    let mut engine: pe::PsychedelicEngine = pe::PsychedelicEngine::init();

    let mut mesh: pe::Mesh3D = pe::Mesh3D::new();
    mesh.vertices = vec![
        pe::engine::Vertex3D {
            position: [-0.5, -0.5, 0.0],
        },
        pe::engine::Vertex3D {
            position: [0.0, 0.5, 0.0],
        },
        pe::engine::Vertex3D {
            position: [0.25, -0.1, 0.0],
        },
    ];

    engine.load_mesh(mesh);
    engine.start();
}
```
That's boring...
I know.

### So what comes next?
* Rewrite core componets with Ash Vulkan Wrapper for more low level control, especially for custom shading with Materials
* Dynamic Materials and Shaders (GLSL) to customize
* 3D Rendering Pipeline: making Vertex3D intentional
* Proper Mesh Input: loading all kinds of geometry files
* Map System: loading more than one map
* Physics: Static- and Rigidbodies
* Sound: 3D Sound Engine
* UI: Handy UI System

And more (hopefully)
