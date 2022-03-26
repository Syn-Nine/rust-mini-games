use super::*;
use crate::mgfw::log;

struct PhysicsSystemData {
    frame: usize,
}

pub struct PhysicsSystem {
    data: *mut PhysicsSystemData,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl PhysicsSystem {
    pub fn new(mgr: &mut CacheManager) -> PhysicsSystem {
        log(format!("Constructing PhysicsSystem"));
        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<PhysicsSystemData>();
        let data = mgr.allocate(sz_bytes) as *mut PhysicsSystemData;

        PhysicsSystem { data }
    }

    fn skip_entity(&self, idx: usize, world: &World) -> bool {
        let ent = world.get_entities();
        if !ent.is_active(idx) || !ent.has_component(idx, COMPONENT_PHYSICS) {
            return true;
        }
        false
    }

    pub fn update(&mut self, world: &mut World, micros: u128) -> bool {
        let expect_blown = false;
        let data = self.get_data_ref_mut();
        let ent = world.get_entities();
        let pcm = world.get_manager_position();
        let acm = world.get_manager_angle();
        let phcm = world.get_manager_physics();

        let span = ent.get_id_span();
        for i in span.first..=span.last {
            if self.skip_entity(i, world) {
                continue;
            }

            // Amortize workload
            match data.frame % 2 {
                // priority 1
                0 => {
                    let dt = micros as f32 * 1.0e-6 * 2.0; // 150hz
                    let accel = phcm.get_acceleration(i);
                    let mut vel = phcm.get_velocity(i);
                    let mut pos = pcm.get_position(i);

                    pos.x += vel.x * dt;
                    pos.y += vel.y * dt;
                    vel.x += accel.x * dt;
                    vel.y += accel.y * dt;

                    pcm.set_position(i, pos.x, pos.y);
                    phcm.set_velocity(i, vel.x, vel.y);

                    let avel = phcm.get_angular_velocity(i);
                    let mut ang = acm.get_angle(i);

                    ang += avel * dt;
                    acm.set_angle(i, ang);
                }

                // priority 2
                1 => (),

                //
                _ => (),
            }
        }
        data.frame += 1;
        expect_blown
    }

    fn get_data_ref_mut(&self) -> &mut PhysicsSystemData {
        unsafe { &mut *(self.data.offset(0)) }
    }

    fn get_data_ref(&self) -> &PhysicsSystemData {
        unsafe { &*(self.data.offset(0)) }
    }
}
