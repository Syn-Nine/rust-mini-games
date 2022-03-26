use crate::mgfw;
//use std::process::exit;
use crate::game::track;
use crate::game::menu;

#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
    pub track_ref: std::boxed::Box<Vec<track::Track>>
}

pub struct GameData {
    pub heap: *mut GameDataHeap,
    pub frame: u8,
    pub ready: bool,
    pub menu_data: menu::MenuData,
    pub track_data: track::TrackData,
    pub started: bool,
}

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    cache.started = false;
    
    track::initialize(cache, heap, world);
    menu::initialize(cache, heap, world);
}


// this gets called by MGFW with input events
#[rustfmt::skip]
pub fn event(
    cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {

    if menu::MENU_PLAYING == cache.menu_data.menu {
        track::event(cache, heap, world, event_id)
    } else {
        menu::event(cache, heap, world, event_id)
    }    
}



pub fn shutdown(_cache: &mut GameData, heap: &mut GameDataHeap) {
    // deallocate and overwrite existing memory
    *heap = GameDataHeap::default();

    // re-box and consume
    //let _temp = unsafe { Box::from_raw(cache.heap) };
}

// this gets called by MGFW at 1200hz
#[rustfmt::skip]
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    cache.frame = (cache.frame + 1) % 128;

    if !cache.ready {
        if 127 == cache.frame {
            cache.ready = true;
        }
        return false;
    }

    if menu::MENU_PLAYING == cache.menu_data.menu {
        track::update(cache, heap, world)
    } else {
        menu::update(cache, heap, world)
    }
}
