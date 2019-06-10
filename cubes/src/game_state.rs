use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Vector3, Transform},
    ecs::prelude::World,
    prelude::{Builder, GameData, SimpleState, StateData},
    renderer::{
        camera,
        light,
        mtl,
        palette::{LinSrgba, Srgb},
        rendy::{
            mesh::{Normal, Position, Tangent, TexCoord},
            texture,
        },
        shape::Shape,
        types,
    },
    window::ScreenDimensions,
};

pub struct CubeGameState;

impl SimpleState for CubeGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.add_floor_entity(data.world);
        self.add_wall_x_entity(data.world);
        self.add_wall_z_entity(data.world);

        self.add_light_entity(data.world, Vector3::new(-1.0, -1.0, -0.2));

        self.add_camera_entity(data.world);
        self.add_sphere_entity(data.world);
    }
}

impl CubeGameState {
    fn add_camera_entity(&self, world: &mut World) {
        let mut t = Transform::default();
        t.set_translation_xyz(10.0, 10.0, 20.0);
        t.face_towards(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

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

    fn add_sphere_entity(&self, world: &mut World) {
        let mut t = Transform::default();
        t.set_translation_xyz(3.0, 3.0, 3.0);

        let mesh = {
            let sphere_mesh_data: types::MeshData = Shape::Sphere(32, 32)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into();

            create_mesh(world, sphere_mesh_data)
        };

        let mat = create_material(world, LinSrgba::new(0.0, 0.0, 1.0, 1.0), 0.8, 0.2);

        world.create_entity().with(t).with(mesh).with(mat).build();
    }

    fn add_floor_entity(&self, world: &mut World) {
        let mut t = Transform::default();
        //t.set_translation_y(-1.0);
        t.append_rotation_x_axis((-90.0f32).to_radians());

        let mesh = {
            let plane_mesh_data: types::MeshData = Shape::Plane(None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                    10.0, 10.0, 10.0, // Scale
                )))
                .into();

            create_mesh(world, plane_mesh_data)
        };

        let mat = create_material(world, LinSrgba::new(0.0, 1.0, 0.0, 0.1), 0.0, 1.0);

        world.create_entity().with(t).with(mesh).with(mat).build();
    }

    fn add_wall_x_entity(&self, world: &mut World) {
        let mut t = Transform::default();
        t.append_rotation_y_axis((90.0f32).to_radians());

        let mesh = {
            let plane_mesh_data: types::MeshData = Shape::Plane(None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                    10.0, 10.0, 10.0, // Scale
                )))
                .into();

            create_mesh(world, plane_mesh_data)
        };

        let mat = create_material(world, LinSrgba::new(1.0, 0.0, 0.0, 0.1), 0.0, 1.0);

        world.create_entity().with(t).with(mesh).with(mat).build();
    }

    fn add_wall_z_entity(&self, world: &mut World) {
        let t = Transform::default();

        let mesh = {
            let plane_mesh_data: types::MeshData = Shape::Plane(None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                    10.0, 10.0, 10.0, // Scale
                )))
                .into();

            create_mesh(world, plane_mesh_data)
        };

        let mat = create_material(world, LinSrgba::new(0.0, 0.0, 1.0, 0.1), 0.0, 1.0);

        world.create_entity().with(t).with(mesh).with(mat).build();
    }

    fn add_light_entity(&self, world: &mut World, direction: Vector3<f32>) {
        let mut t = Transform::default();
        t.set_translation(Vector3::new(6.0, 3.0, 3.0));

        let light: light::Light = light::DirectionalLight {
            color: Srgb::new(1.0, 1.0, 1.0),
            direction: direction.normalize(),
            intensity: 5.0,
        }
        .into();

        //let light: light::Light = light::PointLight {
        //    color: Srgb::new(1.0, 1.0, 1.0),
        //    radius: 0.10,
        //    intensity: 0.5,
        //    smoothness: 20.0,
        //}
        //.into();

        world
            .create_entity()
            .with(light)
            .with(t)
            .build();
    }
}

fn create_mesh(world: &World, mesh_data: types::MeshData) -> Handle<types::Mesh> {
    // Mesh creation
    let loader = world.read_resource::<Loader>();
    let asset_storage = world.read_resource::<AssetStorage<types::Mesh>>();

    let mesh = loader.load_from_data(mesh_data, (), &asset_storage);

    mesh
}

fn create_material(
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
