pub struct Camera3D {
    pub extent: Object3D,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera3D {
    pub fn new() -> Self {
        return Self {
            extent: Object3D::new(),
            fov: 90.0,
            near: 0.1,
            far: 1000.0,
        };
    }
}
