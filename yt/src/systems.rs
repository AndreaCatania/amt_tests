use amethyst::{
    assets::Handle,
    core::{Time, Transform},
    ecs::{Resources, Join, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    renderer::types::Mesh,
    input::{InputEvent, InputHandler, StringBindings,},
    shrev::{EventChannel,ReaderId,},
};

use crate::game_state::{
    Motion,
    Tool,
};

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

const MOUSE_SENSITIVITY: f32 = 20.0;

#[derive(Debug, Default)]
pub struct ToolSystem{
    event_reader: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl<'s> System<'s> for ToolSystem {
    type SystemData = (
        ReadExpect<'s, Time>,
        ReadExpect<'s, EventChannel<InputEvent<StringBindings>>>,
        ReadStorage<'s, Tool>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (time, input_events, tools, mut transforms): Self::SystemData){

        let mut m_motion_x = 0.0;
        let mut m_motion_y = 0.0;
        for e in input_events.read(self.event_reader.as_mut().unwrap()) {
            if let InputEvent::MouseMoved{delta_x, delta_y} = e {
                m_motion_x = *delta_x;
                m_motion_y = *delta_y;
                break;
            }
        }

        for(transf, _) in (&mut transforms, &tools).join() {

            let mut delta_transf = Transform::default();
            delta_transf.append_rotation_y_axis(m_motion_x.to_radians() * MOUSE_SENSITIVITY * time.delta_seconds());
            delta_transf.append_rotation_z_axis(m_motion_y.to_radians() * MOUSE_SENSITIVITY * time.delta_seconds() * -1.0);
            transf.concat(&delta_transf);
            //transf.append_rotation_y_axis(m_motion_x.to_radians() * MOUSE_SENSITIVITY * time.delta_seconds());
            //transf.append_rotation_z_axis(m_motion_y.to_radians() * MOUSE_SENSITIVITY * time.delta_seconds() * -1.0);

        }
    }

    fn setup(&mut self, resources: &mut Resources){
        Self::SystemData::setup(resources);
        let mut ie = resources.fetch_mut::<EventChannel<InputEvent<StringBindings>>>();
        self.event_reader = Some(ie.register_reader());
    }
}
