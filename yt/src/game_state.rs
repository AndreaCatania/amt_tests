use amethyst::{
    prelude::*,
    assets::{AssetStorage, Handle, Loader},
    core::{
        math::{Translation3, Vector, Vector3},
        shrev::{EventChannel, ReaderId},
        Float, Parent, Time, Transform,
    },
    ecs::{prelude::World, Entity, Join},
    input::{InputEvent, InputHandler, StringBindings},
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

#[derive(Default)]
pub struct LoadingState{
    counter: i32,
}

impl SimpleState for LoadingState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Loading on start");
        add_camera_entity(data.world);
        add_light_entity(data.world, Vector3::new(-1.0, -1.0, -1.0));
        add_sphere_entity(data.world);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Loading on stop");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>> ) -> SimpleTrans {
        println!("Loading state on update");

        self.counter += 1;
        if self.counter >= 5 {
            Trans::Switch(Box::new(GamePlayState::default()))
        }else{
            Trans::None
        }
    }
}

#[derive(Default)]
pub struct GamePlayState{
    counter: i32,
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

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>> ) -> SimpleTrans {
        println!("GamePlay state on update");

        self.counter += 1;
        if self.counter == 100 {
            Trans::Push(Box::new(PauseState::default()))
        }else{
            if self.counter == 200 {

                Trans::Quit
            }else{

                Trans::None
            }
        }
    }
}

#[derive(Default)]
pub struct PauseState{
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

fn add_camera_entity( world: &mut World) {
    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(31.0, 7.0, 31.0);
    camera_transform
        .face_towards(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    world
        .create_entity()
        .with(camera_transform.clone())
        .with(camera::Camera::standard_3d(width, height))
        .build();
}

fn add_light_entity(world: &mut World, direction: Vector3<f32>) {
    let mut t = Transform::default();
    t.set_translation(Vector3::new(6.0, 6.0, 6.0));

    let light: light::Light = light::DirectionalLight {
        color: Srgb::new(1.0, 1.0, 1.0),
        direction: direction.normalize(),
        intensity: 5.0,
    }
        .into();

    world.create_entity().with(light).with(t).build();
}

fn add_sphere_entity(world: &mut World) {
    let mesh = {
        let radius = 1.0;
        let mesh_data: types::MeshData = Shape::Sphere(32, 32)
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                radius, radius, radius,
            )))
            .into();

        create_mesh(world, mesh_data)
    };

    let mat = create_material(world, LinSrgba::new(1.0, 1.0, 1.0, 1.0), 0.0, 1.0);

    let t = Transform::default();
    world
        .create_entity()
        .with(t)
        .with(mesh)
        .with(mat)
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