use amethyst::{
    assets::Handle,
    core::{Time, Transform},
    ecs::{Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    renderer::types::Mesh,
};

use crate::game_state::Motion;

#[derive(Debug, Default)]
pub struct MotionSystem;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        ReadExpect<'s, Time>,
        ReadStorage<'s, Motion>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (time, motions, mut transforms): Self::SystemData) {
        for (motion, trsf) in (&motions, &mut transforms).join() {
            trsf.prepend_translation_y(motion.speed * time.delta_seconds());
        }
    }
}
