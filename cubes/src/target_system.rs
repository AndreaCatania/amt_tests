use crate::components::*;

use amethyst::{
    core::Time,
    ecs::{
        Join, System, Entities, ReadStorage, WriteStorage, ReadExpect,
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

pub struct TargetSystem;

impl TargetSystem{
    pub fn new() -> Self {
        TargetSystem{

        }
    }
}
