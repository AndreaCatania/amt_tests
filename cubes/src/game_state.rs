use amethyst::{
    ecs::prelude::World,
    prelude::{GameData, SimpleState, StateData, Builder},
    renderer::{
        camera,
        types,
        mtl,
        rendy::{
            mesh::{
                MeshBuilder,
                Position,
                Normal,
                TexCoord,
                Tangent,
                Indices,
            },
            texture,
        },
        light::{
            Light,
            SunLight,
        },
        palette::{Srgb,LinSrgba,},
        shape::Shape,
    },
    core::{
        Transform,
        math::Vector3,
    },
    assets::{Loader, Handle, AssetStorage, AssetLoaderSystemData},
    window::ScreenDimensions,
    
};

pub struct CubeGameState;

impl SimpleState for CubeGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.add_camera_entity(data.world);
        self.add_cube_entity(data.world);
        self.add_sun_entity(data.world, Vector3::new(1.0, -1.0, 1.0));
        self.add_sun_entity(data.world, Vector3::new(-1.0, -1.0, -1.0));
        self.add_sun_entity(data.world, Vector3::new(-1.0, 1.0, -1.0));
    }
}

impl CubeGameState {

    fn add_camera_entity(&self, world: &mut World) {

        let mut t = Transform::default();
        t.set_translation_xyz(0.0, 0.0, -10.0);

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        world
            .create_entity()
            .with(t)
            .with(camera::Camera::standard_3d(width, height))
            .build();
    }
    
    fn add_cube_entity(&self, world: &mut World) {

        let t = Transform::default();

        let (mesh, mat) = create_cube_mesh(world);

        world
            .create_entity()
            .with(t)
            .with(mesh)
            .with(mat)
            .build();
    }

    fn add_sun_entity(&self, world: &mut World, direction: Vector3<f32>){

        let sun : Light = SunLight {
            angle: 0.0,
            color: Srgb::new(1.0, 1.0, 1.0),
            direction: direction.normalize(),
            intensity: 100000.0,
        }.into();

        world
            .create_entity()
            .with(sun)
            .with(Transform::default())
            .build();
    }
}

fn create_cube_mesh(world: &World) -> (Handle<types::Mesh>, Handle<mtl::Material>){

    // Mesh creation
    let loader = world.read_resource::<Loader>();
    let asset_storage = world.read_resource::<AssetStorage<types::Mesh>>();

    let mesh = loader.load_from_data(
                Shape::Sphere(32, 32)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
                (),
                &asset_storage
            );
    
    // Material creation
    let asset_storage = world.read_resource::<AssetStorage<types::Texture>>();
    let albedo = loader.load_from_data(
        texture::palette::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 1.0)).into(),
        (),
        &asset_storage
    );

    let asset_storage = world.read_resource::<AssetStorage<mtl::Material>>();
    let mat_defaults = world.read_resource::<mtl::MaterialDefaults>().0.clone();
    let material = loader.load_from_data(
        mtl::Material{
            albedo,
            ..mat_defaults.clone()
        },
        (),
        &asset_storage
    );

    (mesh, material)
}