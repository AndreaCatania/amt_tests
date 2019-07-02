
use amethyst::{
    assets::Handle,
    ecs::{
        Component, VecStorage,
    },
    renderer::{
        mtl::Material,
    }
};

pub struct SafeZone{
    pub overlap_count: i32,
    pub activation_timer: f32,
}

impl Default for SafeZone{
    fn default() -> Self {
        SafeZone{
            overlap_count: 0,
            activation_timer: 0.0
        }
    }
}

impl Component for SafeZone{
    type Storage = VecStorage<Self>;
}

pub struct SafeZoneAssets{
    pub idle: Handle<Material>,
    pub active: Handle<Material>,
}
