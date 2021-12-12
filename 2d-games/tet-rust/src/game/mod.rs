mod game;

use crate::mgfw;
use crate::mgfw::*;

pub struct GameWrapper {
    data: *mut game::GameData,
    // WARNING: Anything below this line is not in cache!
}

impl GameWrapper {
    #[rustfmt::skip]
    pub fn new(mgr: &mut mgfw::cache::CacheManager) -> GameWrapper {
        log(format!("Constructing Game"));
        let data = mgr.allocate(std::mem::size_of::<game::GameData>()) as *mut game::GameData;
        let cache: &mut game::GameData = unsafe { &mut *(data.offset(0)) };
        cache.heap = Box::into_raw(Box::new(game::GameDataHeap::default()));
        GameWrapper { data }
    }

    pub fn initialize(&mut self, world: &mut mgfw::ecs::World) {
        log(format!("Initializing Game"));
        game::initialize(self.get_cache_ref_mut(), self.get_heap_ref_mut(), world);
    }

    pub fn update(&mut self, world: &mut mgfw::ecs::World, _micros: u128) -> bool {
        game::update(self.get_cache_ref_mut(), self.get_heap_ref_mut(), world)
    }

    pub fn event(&mut self, world: &mut mgfw::ecs::World, event_id: u8) -> bool {
        game::event(
            self.get_cache_ref_mut(),
            self.get_heap_ref_mut(),
            world,
            event_id,
        )
    }

    pub fn shutdown(&mut self) {
        log(format!("Shutdown Game"));
        game::shutdown(self.get_cache_ref_mut(), self.get_heap_ref_mut());
    }

    fn get_cache_ref_mut(&self) -> &mut game::GameData {
        unsafe { &mut *(self.data.offset(0)) }
    }

    fn get_heap_ref_mut(&self) -> &mut game::GameDataHeap {
        unsafe { &mut *(self.get_cache_ref_mut().heap) }
    }
}
