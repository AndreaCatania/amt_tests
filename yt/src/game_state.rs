use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        math::{Translation3, Vector, Vector3},
        shrev::{EventChannel, ReaderId},
        Float, Parent, Time, Transform,
    },
    ecs::{prelude::World, Component, DenseVecStorage, Entity, NullStorage},
    input::{InputEvent, InputHandler, StringBindings},
    prelude::*,
    renderer::{
        camera, light, mtl,
        palette::{LinSrgba, Srgb},
        rendy::{
            mesh::{Normal, Position, Tangent, TexCoord},
            texture,
        },
        shape::Shape,
        types, Transparent,
    },
    window::ScreenDimensions,
    StateEvent,
};

use rand::prelude::*;

#[derive(Default)]
pub struct LoadingState {
    counter: i32,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Loading on start");

        data.world.add_resource(MyHandleStorage::default());

        add_camera_entity(data.world);
        add_light_entity(data.world, Vector3::new(-1.0, -1.0, -1.0));
        add_sphere_entity(data.world);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Loading on stop");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("Loading state on update");

        self.counter += 1;
        if self.counter >= 5 {
            Trans::Switch(Box::new(GamePlayState::default()))
        } else {
            Trans::None
        }
    }
}

#[derive(Default)]
pub struct GamePlayState {
    time_bank: f32,
    last_added_mesh_type: bool,
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("GamePlay on start");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("GamePlay on stop");
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("GamePlay on pause");
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("GamePlay on resume");
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.time_bank += { data.world.read_resource::<Time>().delta_seconds() };

        if self.time_bank > 1.0 {
            self.time_bank -= 1.0;

            if self.last_added_mesh_type {
                add_cube_entity(data.world);
            } else {
                add_sphere_entity(data.world);
            }

            self.last_added_mesh_type = !self.last_added_mesh_type;
        }

        Trans::None
    }
}

#[derive(Default)]
pub struct PauseState {
    counter: i32,
}

impl SimpleState for PauseState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("PauseState on start");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("PauseState on stop");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        println!("PauseState on update");

        self.counter += 1;
        if self.counter >= 50 {
            Trans::Pop
        } else {
            Trans::None
        }
    }
}

// Utilities ------------------------------------

fn add_camera_entity(world: &mut World) {
    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(31.0, 7.0, 31.0);
    camera_transform.face_towards(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    world
        .create_entity()
        .with(camera_transform)
        .with(camera::Camera::standard_3d(width, height))
        .build();
}

fn add_light_entity(world: &mut World, direction: Vector3<f32>) {
    let mut light_transform = Transform::default();
    light_transform.set_translation(Vector3::new(6.0, 6.0, 6.0));

    let light: light::Light = light::DirectionalLight {
        color: Srgb::new(1.0, 1.0, 1.0),
        direction: direction.normalize(),
        intensity: 5.0,
    }
    .into();

    world
        .create_entity()
        .with(light)
        .with(light_transform)
        .build();
}

fn add_sphere_entity(world: &mut World) {
    let mesh = {
        let mut my_handle_storage = world.write_resource::<MyHandleStorage>();

        my_handle_storage
            .sphere_mesh
            .get_or_insert_with(|| {
                let radius = 1.0;
                let mesh_data: types::MeshData = Shape::Sphere(32, 32)
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                        radius, radius, radius,
                    )))
                    .into();

                create_mesh(world, mesh_data)
            })
            .clone()
    };

    let mat = {
        let mut my_handle_storage = world.write_resource::<MyHandleStorage>();

        my_handle_storage
            .material
            .get_or_insert_with(|| {
                create_material(world, LinSrgba::new(1.0, 1.0, 1.0, 1.0), 0.0, 1.0)
            })
            .clone()
    };

    let mut sphere_transform = Transform::default();

    let mut rng = rand::thread_rng();
    sphere_transform.set_translation_x(rng.gen_range(-3.0, 3.0));
    sphere_transform.set_translation_y(rng.gen_range(-3.0, 3.0));
    sphere_transform.set_translation_z(rng.gen_range(-3.0, 3.0));

    world
        .create_entity()
        .with(sphere_transform)
        .with(mesh)
        .with(mat)
        .with(Motion::new(-1.0 * (2.0 + 10.0 * rng.gen::<f32>())))
        .build();
}

fn add_cube_entity(world: &mut World) {
    let mesh = {
        let mut my_handle_storage = world.write_resource::<MyHandleStorage>();

        my_handle_storage
            .cube_mesh
            .get_or_insert_with(|| {
                let radius = 1.0;
                let mesh_data: types::MeshData = Shape::Cube
                    .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                        radius, radius, radius,
                    )))
                    .into();

                create_mesh(world, mesh_data)
            })
            .clone()
    };

    let mat = {
        let mut my_handle_storage = world.write_resource::<MyHandleStorage>();

        my_handle_storage
            .material
            .get_or_insert_with(|| {
                create_material(world, LinSrgba::new(1.0, 1.0, 1.0, 1.0), 0.0, 1.0)
            })
            .clone()
    };

    let mut sphere_transform = Transform::default();

    let mut rng = rand::thread_rng();
    sphere_transform.set_translation_x(rng.gen_range(-3.0, 3.0));
    sphere_transform.set_translation_y(rng.gen_range(-3.0, 3.0));
    sphere_transform.set_translation_z(rng.gen_range(-3.0, 3.0));

    world
        .create_entity()
        .with(sphere_transform)
        .with(mesh)
        .with(mat)
        .with(Motion::new(2.0 + 10.0 * rng.gen::<f32>()))
        .build();
}

pub fn create_mesh(world: &World, mesh_data: types::MeshData) -> Handle<types::Mesh> {
    // Mesh creation
    let loader = world.read_resource::<Loader>();
    let asset_storage = world.read_resource::<AssetStorage<types::Mesh>>();

    let mesh = loader.load_from_data(mesh_data, (), &asset_storage);

    mesh
}

pub fn create_material(
    world: &World,
    color: LinSrgba,
    metallic: f32,
    roughness: f32,
) -> Handle<mtl::Material> {
    let loader = world.read_resource::<Loader>();

    // Material creation
    let asset_storage = world.read_resource::<AssetStorage<types::Texture>>();
    let albedo = loader.load_from_data(
        texture::palette::load_from_linear_rgba(color).into(),
        (),
        &asset_storage,
    );

    let metallic_roughness = loader.load_from_data(
        texture::palette::load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0))
            .into(),
        (),
        &asset_storage,
    );

    let asset_storage = world.read_resource::<AssetStorage<mtl::Material>>();
    let mat_defaults = world.read_resource::<mtl::MaterialDefaults>().0.clone();

    let material = loader.load_from_data(
        mtl::Material {
            albedo,
            metallic_roughness,
            ..mat_defaults
        },
        (),
        &asset_storage,
    );

    material
}

#[derive(Default)]
struct MyHandleStorage {
    pub sphere_mesh: Option<Handle<types::Mesh>>,
    pub cube_mesh: Option<Handle<types::Mesh>>,
    pub material: Option<Handle<mtl::Material>>,
}

pub struct Motion {
    pub speed: f32,
}

impl Motion {
    pub fn new(speed: f32) -> Self {
        Motion { speed }
    }
}

impl Component for Motion {
    type Storage = DenseVecStorage<Self>;
}
