use crate::components::*;

use amethyst::{
    core::Time,
    ecs::{
        Join, System, Entities, Entity, ReadStorage, WriteStorage, ReadExpect,
    },
    phythyst::{
        servers::*,
        objects::*,
    },
    assets::{
        Handle,
    },
    renderer::{
        mtl::Material,
    }
};

pub struct ImpulseSystem{

}

impl ImpulseSystem{
    pub fn new() -> Self {
        ImpulseSystem {

        }
    }
}

impl<'s> System<'s> for ImpulseSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, RBodyPhysicsServer<f32>>,
        ReadStorage<'s, PhysicsBodyTag>,
        WriteStorage<'s, PhysicalImpulse>,
    );

    fn run(&mut self, (entities, body_server, bodies, mut impulses): Self::SystemData) {

        let mut entities_with_impulse = Vec::<Entity>::new();
        for (entity, body, impulse) in (&*entities, &bodies, &impulses).join() {

            body_server.apply_impulse(*body, &impulse.impulse);

            entities_with_impulse.push(entity);
        }

        for e in entities_with_impulse.iter() {

            impulses.remove(*e);
        }
    }
}
