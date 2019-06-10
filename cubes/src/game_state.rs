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
                Indices,
            },
            texture,
        },
        light::{
            Light,
            SunLight,
        },
        palette::{Srgb,LinSrgba,},
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
            intensity: 100.0,
        }.into();

        world
            .create_entity()
            .with(sun)
            .with(Transform::default())
            .build();
    }
    
}

fn create_cube_mesh(world: &World) -> (Handle<types::Mesh>, Handle<mtl::Material>){

    // Mesh preparation

    let positions: Vec<Position> = vec![
        
        Position([0.0, 0.0, 0.0]),
        Position([0.0, 1.0, 0.0]),
        Position([1.0, 0.0, 1.0]),
    ];

    let normals: Vec<Normal> = vec![

        Normal([0.0, 0.0, 1.0]),
        Normal([0.0, 0.0, 1.0]),
        Normal([0.0, 0.0, 1.0]),
    ];

    let tex_coords: Vec<TexCoord> = vec![
        TexCoord([0.0, 0.0]),
        TexCoord([0.0, 1.0]),
        TexCoord([1.0, 0.0]),
    ];

    let indices = vec![0, 1, 2];

    // Mesh creation
    let loader = world.read_resource::<Loader>();
    let asset_storage = world.read_resource::<AssetStorage<types::Mesh>>();

    let mesh = loader.load_from_data(
                types::MeshData(
                    MeshBuilder::new()
                        .with_vertices(positions)
                        .with_vertices(normals)
                        .with_vertices(tex_coords)
                        .with_indices(Indices::U16(indices.into())),
                ),
                (),
                &asset_storage
            );
    
    // Material creation

    let asset_storage = world.read_resource::<AssetStorage<types::Texture>>();
    let albedo = loader.load_from_data(
        texture::palette::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
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