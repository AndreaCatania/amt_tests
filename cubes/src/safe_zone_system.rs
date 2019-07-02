
use crate::safe_zone::*;

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

pub struct SafeZoneSystem;

impl SafeZoneSystem {
    pub fn new() -> Self {
        SafeZoneSystem {

        }
    }
}

/// #IMPORTANT
/// Physics substepping is not yet supported but this system must be executed in subscripts
impl<'s> System<'s> for SafeZoneSystem {

    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, Time>,
        ReadExpect<'s, AreaPhysicsServer>,
        ReadExpect<'s, SafeZoneAssets>,
        ReadStorage<'s, PhysicsAreaTag>,
        WriteStorage<'s, SafeZone>,
        WriteStorage<'s, Handle<Material>>,
    );

    fn run(&mut self, (entities, time, area_server, safe_zone_assets, areas, mut safe_zones, mut mats): Self::SystemData) {

        for (entity, area, safe_zone, mat) in (&*entities, &areas, &mut safe_zones, &mut mats).join() {

            let events = area_server.0.overlap_events(*area);

            for e in events {

                match e {
                    OverlapEvent::Enter(_) => {
                        safe_zone.overlap_count += 1;
                    }
                    OverlapEvent::Exit(_) => {
                        safe_zone.overlap_count -= 1;
                    }
                }
            }

            if safe_zone.overlap_count > 0 {

                safe_zone.activation_timer = 2.0;
            }

            safe_zone.activation_timer -= time.delta_seconds();

            let safe_zone_mat = if safe_zone.activation_timer > 0.0 {
                safe_zone_assets.active.clone()
            }else{
                safe_zone_assets.idle.clone()
            };

            *mat = safe_zone_mat;
        }
    }
}