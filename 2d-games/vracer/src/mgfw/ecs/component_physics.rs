use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

pub struct PhysicsComponentManagerData {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub angular_velocity: f32,
}

pub struct PhysicsComponentManager {
    data: *mut PhysicsComponentManagerData,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl PhysicsComponentManager {
    pub fn new(mgr: &mut CacheManager) -> PhysicsComponentManager {
        log(format!("Constructing PhysicsComponentManager"));
        let sz_bytes = std::mem::size_of::<PhysicsComponentManagerData>() * ENTITY_SZ;
        PhysicsComponentManager {
            data: mgr.allocate(sz_bytes) as *mut PhysicsComponentManagerData,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_acceleration(i, 0.0, 0.0);
            self.set_velocity(i, 0.0, 0.0);
            self.set_angular_velocity(i, 0.0);
        }
    }

    pub fn get_velocity(&self, idx: usize) -> Velocity {
        let data = self.get_data_ref(idx);
        data.velocity.clone()
    }

    pub fn get_acceleration(&self, idx: usize) -> Acceleration {
        let data = self.get_data_ref(idx);
        data.acceleration.clone()
    }

    pub fn get_angular_velocity(&self, idx: usize) -> f32 {
        let data = self.get_data_ref(idx);
        data.angular_velocity
    }

    pub fn set_angular_velocity(&self, idx: usize, val: f32) {
        let data = self.get_data_ref_mut(idx);
        data.angular_velocity = val;
    }

    pub fn set_velocity(&self, idx: usize, x: f32, y: f32) {
        let data = self.get_data_ref_mut(idx);
        data.velocity.x = x;
        data.velocity.y = y;
    }

    pub fn set_acceleration(&self, idx: usize, x: f32, y: f32) {
        let data = self.get_data_ref_mut(idx);
        data.acceleration.x = x;
        data.acceleration.y = y;
    }

    pub fn get_data_ref_mut(&self, idx: usize) -> &mut PhysicsComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    pub fn get_data_ref(&self, idx: usize) -> &PhysicsComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
