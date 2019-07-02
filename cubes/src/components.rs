
use amethyst::{
    assets::Handle,
    ecs::{
        Component, HashMapStorage, NullStorage, VecStorage,
    },
    renderer::{
        mtl::Material,
    },
    core::math::*,
};

pub struct PhysicalImpulse{
    pub impulse: Vector3<f32>,
}

impl PhysicalImpulse {
    pub fn new(impulse: Vector3<f32>) -> Self {
        PhysicalImpulse{impulse}
    }
}

impl Component for PhysicalImpulse {
    type Storage = HashMapStorage<Self>;
}

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
