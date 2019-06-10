
use amethyst::{
    ecs::prelude::{Component, DenseVecStorage}
};

pub struct Cube{};

impl Component for Cube{
    type Storage = DenseVecStorage<Self>();
};