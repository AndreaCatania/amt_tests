use crate::{
    safe_zone::*,
    safe_zone_system::SafeZoneSystem,
};

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        math::{Vector, Vector3},
        Time, Transform, Float,
    },
    ecs::prelude::World,
    input::{InputEvent, InputHandler, StringBindings},
    phythyst::{objects::*, servers::*},
    prelude::{Builder, GameData, SimpleState, SimpleTrans, StateData, Trans},
    renderer::{
        camera, light, mtl,
        palette::{LinSrgba, Srgb},
        rendy::{
            mesh::{Normal, Position, Tangent, TexCoord},
            texture,
        },
        Transparent,
        shape::Shape,
        types,
    },
    window::ScreenDimensions,
    StateEvent,
};

use rand::prelude::*;

const SAFE_ZONE_RADIUS :f32 = 10.0;

pub struct CubeGameState {
    bullet_fired: bool,
    bullet_shape: Option<PhysicsShapeTag>,
    platform_shape: Option<PhysicsShapeTag>,
    safe_zone_area: Option<PhysicsShapeTag>,
    camera_transform: Transform,
}

impl CubeGameState {
    pub fn new() -> Self {
        CubeGameState {
            bullet_fired: false,
            bullet_shape: None,
            platform_shape: None,
            safe_zone_area: None,
            camera_transform: Transform::default(),
        }
    }
}

impl SimpleState for CubeGameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut transf = Transform::default();

        self.initialize_bullet_shape(data.world, 0.5);
        self.initialize_platform_shape(data.world);
        self.initialize_safe_zone(data.world);

        transf.append_rotation_x_axis(90.0f32.to_radians());
        self.add_cube(data.world, &transf);

        transf.set_translation_xyz(0.0, 10.0, -18.0);
        transf.append_rotation_x_axis(25.0f32.to_radians());
        self.add_cube(data.world, &transf);

        transf.set_translation_xyz(0.0, -10.0, 13.0);
        transf.append_rotation_x_axis(-55.0f32.to_radians());
        self.add_cube(data.world, &transf);

        transf.set_translation_xyz(0.0, -15.0, 9.0);
        transf.set_rotation_x_axis(0.0f32.to_radians());
        self.add_cube(data.world, &transf);

        self.add_light_entity(data.world, Vector3::new(-1.0, -1.0, -1.0));

        self.add_camera_entity(data.world);

        self.add_safe_zone(data.world, &Transform::default());
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let mut want_to_fire = false;
        {
            let ih = data.world.read_resource::<InputHandler<StringBindings>>();

            want_to_fire = ih.action_is_down("shot").unwrap();
        }

        if want_to_fire {
            if !self.bullet_fired {
                self.bullet_fired = true;

                let impulse = self.camera_transform.rotation() * Vector3::z();
                let impulse = Vector3::new(impulse.x.into(), impulse.y.into(), impulse.z.into());
                let impulse = impulse * -1.0 * 100.0;

                self.add_bullet_entity(
                    data.world,
                    &self.camera_transform,
                    0.5,
                    &impulse,
                );
            }
        } else {
            self.bullet_fired = false;
        }

        Trans::None
    }
}

impl CubeGameState {

    fn initialize_bullet_shape(&mut self, world: &mut World, radius: f32) {
        let mut shape_server = world.write_resource::<ShapePhysicsServer<f32>>();
        let shape_desc = ShapeDesc::Sphere { radius };
        self.bullet_shape = Some(shape_server.create_shape(&shape_desc));
    }

    fn initialize_platform_shape(&mut self, world: &mut World) {
        let mut shape_server = world.write_resource::<ShapePhysicsServer<f32>>();
        let shape_desc = ShapeDesc::<f32>::Cube {
            half_extents: Vector3::new(10.0, 10.0, 0.3),
        };
        self.platform_shape = Some(shape_server.create_shape(&shape_desc));
    }

    fn initialize_safe_zone(&mut self, world: &mut World) {
        {
            let mut shape_server = world.write_resource::<ShapePhysicsServer<f32>>();
            let shape_desc = ShapeDesc::<f32>::Sphere {
                radius: SAFE_ZONE_RADIUS,
            };
            self.safe_zone_area = Some(shape_server.create_shape(&shape_desc));
        }

        let safe_zone_assets = SafeZoneAssets{
            idle: create_material(
                world,
                LinSrgba::new(0.2, 0.1, 0.1, 0.05),
                0.0,
                1.0,
            ),
            active: create_material(
                world,
                LinSrgba::new(1.0, 0.0, 0.0, 0.6),
                0.0,
                1.0,
            )
        };

        world.add_resource(safe_zone_assets);
    }

    fn add_camera_entity(&mut self, world: &mut World) {
        self.camera_transform.set_translation_xyz(35.0, 20.0, 35.0);
        self.camera_transform
            .face_towards(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        world
            .create_entity()
            .with(self.camera_transform.clone())
            .with(camera::Camera::standard_3d(width, height))
            .build();
    }

    fn add_bullet_entity(
        &self,
        world: &mut World,
        transform: &Transform,
        radius: f32,
        impulse: &Vector3<f32>,
    ) {
        // Mesh

        let mesh = {
            let sphere_mesh_data: types::MeshData = Shape::Sphere(32, 32)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                    radius, radius, radius,
                )))
                .into();

            create_mesh(world, sphere_mesh_data)
        };

        let mut rng = thread_rng();

        let mat = create_material(
            world,
            LinSrgba::new(rng.gen(), rng.gen(), rng.gen(), 0.2),
            0.3,
            0.7,
        );

        // Rigid body
        let rb = create_rigid_body(
            world,
            &transform,
            self.bullet_shape.unwrap(),
            BodyMode::Dynamic,
            impulse,
        );

        world
            .create_entity()
            .with(transform.clone())
            .with(mesh)
            .with(mat)
            .with(rb)
            .build();
    }

    fn add_safe_zone(&self, world: &mut World, transf: &Transform){

        let mesh = {
            let sphere_mesh_data: types::MeshData = Shape::Sphere(32, 32)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                    SAFE_ZONE_RADIUS, SAFE_ZONE_RADIUS, SAFE_ZONE_RADIUS,
                )))
                .into();

            create_mesh(world, sphere_mesh_data)
        };

        let safe_zone_mat_idle = world.read_resource::<SafeZoneAssets>().idle.clone();

        let area = create_area(world, transf, self.safe_zone_area.unwrap());

        world
            .create_entity()
            .with(mesh)
            .with(safe_zone_mat_idle)
            .with(Transparent::default())
            .with(transf.clone())
            .with(area)
            .with(SafeZone::default())
            .build();
    }

    fn add_cube(&self, world: &mut World, transf: &Transform) {
        let mesh = {
            let plane_mesh_data: types::MeshData = Shape::Cylinder(128usize, Some(1usize))
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                    10.0, 10.0, 0.3, // Scale
                )))
                .into();

            create_mesh(world, plane_mesh_data)
        };

        let mat = create_material(world, LinSrgba::new(0.0, 1.0, 0.0, 1.0), 0.5, 0.5);

        let rb = create_rigid_body(
            world,
            transf,
            self.platform_shape.unwrap(),
            BodyMode::Static,
            &Vector3::zeros(),
        );

        world
            .create_entity()
            .with(transf.clone())
            .with(mesh)
            .with(mat)
            .with(rb)
            .build();
    }

    fn add_light_entity(&self, world: &mut World, direction: Vector3<f32>) {
        let mut t = Transform::default();
        t.set_translation(Vector3::new(6.0, 6.0, 6.0));

        let light: light::Light = light::DirectionalLight {
            color: Srgb::new(1.0, 1.0, 1.0),
            direction: direction.normalize(),
            intensity: 5.0,
        }
        .into();

        //let light: light::Light = light::PointLight {
        //    color: Srgb::new(1.0, 1.0, 1.0),
        //    radius: 20.0,
        //    intensity: 10.5,
        //    smoothness: 20.0,
        //}
        //.into();

        world.create_entity().with(light).with(t).build();
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

fn create_rigid_body(
    world: &World,
    transform: &Transform,
    shape: PhysicsShapeTag,
    body_mode: BodyMode,
    impulse: &Vector3<f32>,
) -> PhysicsBodyTag {
    let mut rigid_body_server = world.write_resource::<RBodyPhysicsServer<f32>>();
    let mut world_server = world.write_resource::<WorldPhysicsServer<f32>>();
    let physics_world = world.read_resource::<PhysicsWorldTag>();

    let desc = RigidBodyDesc {
        mode: body_mode,
        transformation: transform.clone(),
        mass: 1.0,
        shape,
    };

    let body = rigid_body_server.create_body(*physics_world, &desc);

    rigid_body_server.apply_impulse(body, impulse);

    body
}

fn create_area(
    world: &World,
    transform: &Transform,
    shape: PhysicsShapeTag) -> PhysicsAreaTag {

    let mut area_server = world.write_resource::<AreaPhysicsServer>();
    let physics_world = world.read_resource::<PhysicsWorldTag>();

    let area_desc = AreaDesc {
        shape,
        transform: transform.clone(),
    };

    area_server.create_area(*physics_world, &area_desc)
}