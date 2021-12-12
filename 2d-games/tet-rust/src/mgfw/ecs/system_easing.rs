use super::*;
use crate::mgfw::log;

struct EasingSystemData {
    _temp: usize,
}

pub struct EasingSystem {
    data: *mut EasingSystemData,
    // WARNING: Anything below this line is not in cache!
    frame: usize,
}

#[allow(dead_code)]
impl EasingSystem {
    pub fn new(mgr: &mut CacheManager) -> EasingSystem {
        log(format!("Constructing EasingSystem"));
        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<EasingSystemData>();
        let data = mgr.allocate(sz_bytes) as *mut EasingSystemData;

        EasingSystem { data, frame: 0 }
    }

    pub fn update(&mut self, world: &mut World, micros: u128) -> bool {
        let expect_blown = false;

        //let ecm = world.get_manager_easing();
        let span = world.get_manager_easing().get_id_span();
        let dt = micros as f32 * 1.0e-6;

        for e in span.first..=span.last {
            let data = world.get_manager_easing().get_data_ref(e).clone();
            match data.variable {
                EASING_VAR_ALPHA => {
                    let cur = world.entity_get_alpha(data.entity as usize);
                    let mut upd = ease(cur, data.dxdt, dt);
                    let err = (data.end - upd) / data.dxdt;
                    if 0.0 > err {
                        upd = data.end;
                        world.easing_disable(e);
                    }
                    world.entity_set_alpha(data.entity as usize, upd);
                }
                _ => (),
            }
        }

        self.frame += 1;
        expect_blown
    }

    fn get_data_ref_mut(&self) -> &mut EasingSystemData {
        unsafe { &mut *(self.data.offset(0)) }
    }

    fn get_data_ref(&self) -> &EasingSystemData {
        unsafe { &*(self.data.offset(0)) }
    }
}

fn ease(current: f32, dxdt: f32, dt: f32) -> f32 {
    current + dxdt * dt
}
