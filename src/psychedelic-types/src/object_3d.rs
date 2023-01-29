use cgmath::Vector3;

pub struct Object3D {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub size: Vector3<f32>,
}

impl Object3D {
    pub fn new() -> Self {
        return Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            size: Vector3::new(1.0, 1.0, 1.0),
        };
    }
}

impl Default for Object3D {
    fn default() -> Self {
        Self::new()
    }
}
