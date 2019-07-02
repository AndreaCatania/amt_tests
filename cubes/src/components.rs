
use amethyst::{
    assets::Handle,
    ecs::{
        Component, NullStorage, VecStorage,
    },
    renderer::{
        mtl::Material,
    }
};

pub struct SafeZone{
    pub activation_timer: f32,
}

impl Default for SafeZone{
    fn default() -> Self {
        SafeZone{
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

pub struct Bullet{}

impl Default for Bullet{
    fn default() -> Self {
        Bullet{}
    }
}

impl Component for Bullet {
    type Storage = NullStorage<Self>;
}