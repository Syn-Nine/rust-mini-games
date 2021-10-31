use crate::mgfw;


#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
}

pub struct Stone {
    entity: usize,
    suite: u8,
    number: u8,
    active: bool,
}

const NUM_STONES: usize = 144;
const NONE_SELECTED: usize = 999;

const SUITE_CHARACTER: u8 = 0;
const SUITE_NUMBER: u8 = 1;
const SUITE_BONE: u8 = 2;
const SUITE_MASK: u8 = 3;
const SUITE_WIND: u8 = 4;
const SUITE_SEASON: u8 = 5;
const SUITE_FLOWER: u8 = 6;
const BOARD_SZ: usize = 2550;

pub struct GameData {
    pub heap: *mut GameDataHeap,
    frame: u8,
    ready: bool,
    game_timer: f64,
    game_timer_last: i64,
    stones: [Stone; NUM_STONES],
    first: usize,
    selected: usize,
    board: [usize; BOARD_SZ],
    options: i32,
    stuck: bool,
    win: bool,
}


#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.parse_world("assets/world.dat");
    
    cache.selected = NONE_SELECTED;
    cache.frame = 0;
    cache.ready = false;
    cache.game_timer = 0.0;
    cache.options = 0;
    cache.stuck = false;
    cache.win = false;
    cache.game_timer_last = -1;

    for i in 0..NUM_STONES {
        // stone highlight
        let id = world.new_entity();
        world.entity_set_billboard(id, String::from("assets/highlight.png"));
        world.entity_set_scale_xy(id, 30.0, 42.0);
        world.entity_set_visibility(id, false);
        
        // stone
        let id = world.new_entity();
        cache.stones[i].entity = id;
        cache.stones[i].active = true;
        world.entity_set_scale_xy(id, 24.0, 36.0);
        world.entity_set_visibility(id, true);
    }

    cache.first = cache.stones[0].entity;

    let mut svec: Vec<&str> = Vec::new();
    let mut tvec: Vec<u8> = Vec::new();
    let mut nvec: Vec<u8> = Vec::new();

    for _i in 0..4 {
        svec.push("assets/stone_c1.png"); tvec.push(SUITE_CHARACTER); nvec.push(1);
        svec.push("assets/stone_c2.png"); tvec.push(SUITE_CHARACTER); nvec.push(2);
        svec.push("assets/stone_c3.png"); tvec.push(SUITE_CHARACTER); nvec.push(3);
        svec.push("assets/stone_c4.png"); tvec.push(SUITE_CHARACTER); nvec.push(4);
        svec.push("assets/stone_c5.png"); tvec.push(SUITE_CHARACTER); nvec.push(5);
        svec.push("assets/stone_c6.png"); tvec.push(SUITE_CHARACTER); nvec.push(6);
        svec.push("assets/stone_c7.png"); tvec.push(SUITE_CHARACTER); nvec.push(7);
        svec.push("assets/stone_c8.png"); tvec.push(SUITE_CHARACTER); nvec.push(8);
        svec.push("assets/stone_c9.png"); tvec.push(SUITE_CHARACTER); nvec.push(9);
        //
        svec.push("assets/stone_n1.png"); tvec.push(SUITE_NUMBER); nvec.push(1);
        svec.push("assets/stone_n2.png"); tvec.push(SUITE_NUMBER); nvec.push(2);
        svec.push("assets/stone_n3.png"); tvec.push(SUITE_NUMBER); nvec.push(3);
        svec.push("assets/stone_n4.png"); tvec.push(SUITE_NUMBER); nvec.push(4);
        svec.push("assets/stone_n5.png"); tvec.push(SUITE_NUMBER); nvec.push(5);
        svec.push("assets/stone_n6.png"); tvec.push(SUITE_NUMBER); nvec.push(6);
        svec.push("assets/stone_n7.png"); tvec.push(SUITE_NUMBER); nvec.push(7);
        svec.push("assets/stone_n8.png"); tvec.push(SUITE_NUMBER); nvec.push(8);
        svec.push("assets/stone_n9.png"); tvec.push(SUITE_NUMBER); nvec.push(9);
        //
        svec.push("assets/stone_b1.png"); tvec.push(SUITE_BONE); nvec.push(1);
        svec.push("assets/stone_b2.png"); tvec.push(SUITE_BONE); nvec.push(2);
        svec.push("assets/stone_b3.png"); tvec.push(SUITE_BONE); nvec.push(3);
        svec.push("assets/stone_b4.png"); tvec.push(SUITE_BONE); nvec.push(4);
        svec.push("assets/stone_b5.png"); tvec.push(SUITE_BONE); nvec.push(5);
        svec.push("assets/stone_b6.png"); tvec.push(SUITE_BONE); nvec.push(6);
        svec.push("assets/stone_b7.png"); tvec.push(SUITE_BONE); nvec.push(7);
        svec.push("assets/stone_b8.png"); tvec.push(SUITE_BONE); nvec.push(8);
        svec.push("assets/stone_b9.png"); tvec.push(SUITE_BONE); nvec.push(9);
        //
        svec.push("assets/stone_w1.png"); tvec.push(SUITE_WIND); nvec.push(1);
        svec.push("assets/stone_w2.png"); tvec.push(SUITE_WIND); nvec.push(2);
        svec.push("assets/stone_w3.png"); tvec.push(SUITE_WIND); nvec.push(3);
        svec.push("assets/stone_w4.png"); tvec.push(SUITE_WIND); nvec.push(4);
        //
        svec.push("assets/stone_d1.png"); tvec.push(SUITE_MASK); nvec.push(1);
        svec.push("assets/stone_d2.png"); tvec.push(SUITE_MASK); nvec.push(2);
        svec.push("assets/stone_d3.png"); tvec.push(SUITE_MASK); nvec.push(3);
    }

    //
    svec.push("assets/stone_s1.png"); tvec.push(SUITE_SEASON); nvec.push(1);
    svec.push("assets/stone_s2.png"); tvec.push(SUITE_SEASON); nvec.push(1);
    svec.push("assets/stone_s3.png"); tvec.push(SUITE_SEASON); nvec.push(1);
    svec.push("assets/stone_s4.png"); tvec.push(SUITE_SEASON); nvec.push(1);
    //
    svec.push("assets/stone_f1.png"); tvec.push(SUITE_FLOWER); nvec.push(1);
    svec.push("assets/stone_f2.png"); tvec.push(SUITE_FLOWER); nvec.push(1);
    svec.push("assets/stone_f3.png"); tvec.push(SUITE_FLOWER); nvec.push(1);
    svec.push("assets/stone_f4.png"); tvec.push(SUITE_FLOWER); nvec.push(1);
    
    let mut mapping: [usize; NUM_STONES] = [0; NUM_STONES];
    for i in 0..NUM_STONES {
        mapping[i] = i;
    }

    // shuffle
    for i in 0..NUM_STONES {
        let j: usize = (mgfw::rnd() * NUM_STONES as f32).floor() as usize;
        let temp = mapping[j];
        mapping[j] = mapping[i];
        mapping[i] = temp;
    }
    
    for i in 0..NUM_STONES {
        cache.stones[i].suite = tvec[mapping[i]];
        cache.stones[i].number = nvec[mapping[i]];
        let id = cache.stones[i].entity;
        world.entity_set_billboard(id, String::from(svec[mapping[i]]));
    }
    
    // classic board
    cache.board = [
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,
    0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,
    0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    //
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    //
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    //
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    //
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    ];


    let mut sidx: usize = 0;
    for z in 0..5 {
        for x in 0..30 {
            for y in 0..17 {
                let bidx: usize = z * 510 + y * 30 + x;
                if 1 == cache.board[bidx] {
                    let px: f32 = 320.0 - 85.0 - 145.0 + x as f32 * 10.0 - z as f32 * 5.0;
                    let py: f32 = 180.0 - 130.0 + y as f32 * 16.0 - z as f32 * 6.0;
                    cache.board[bidx] = sidx + 1;
                    let eidx = cache.stones[sidx].entity;
                    world.entity_set_position_xy(eidx - 1, px, py);                    
                    world.entity_set_position_xy(eidx, px, py);
                    sidx = sidx + 1;
                }
            }
        }
    }

}


fn check_stuck(cache: &mut GameData) -> bool {

    let mut options: Vec<usize> = Vec::new();
    // check constraints
    for z in 0..5 {
        for x in 0..30 {
            for y in 0..17 {
                let bidx: usize = z * 510 + y * 30 + x;
                if 0 != cache.board[bidx] {
                    let mut found = cache.board[bidx];

                    // check lhs/rhs constraints
                    let mut lhs: bool = false;
                    let mut rhs: bool = false;
                    for yy in 0..3 {
                        let yloc: i32 = y as i32 + yy as i32 - 1;
                        let xloc: i32 = x as i32 - 2;
                        if yloc >= 0 && yloc <= 17 && xloc >= 0 && xloc <= 30 {
                            let nidx: usize = z * 510 + yloc as usize * 30 + xloc as usize;
                            if 0 != cache.board[nidx] {
                                lhs = true;
                            }
                        }
                        let xloc: i32 = x as i32 + 2;
                        if yloc >= 0 && yloc <= 17 && xloc >= 0 && xloc <= 30 {
                            let nidx: usize = z * 510 + yloc as usize * 30 + xloc as usize;
                            if 0 != cache.board[nidx] {
                                rhs = true;
                            }
                        }
                    }
                    if lhs && rhs {
                        found = NONE_SELECTED;
                    }

                    // check z constraint
                    for zz in (z + 1)..5 {
                        for xx in 0..3 {
                            for yy in 0..3 {
                                let yloc: i32 = y as i32 + yy as i32 - 1;
                                let xloc: i32 = x as i32 + xx as i32 - 1;
                                if yloc >= 0 && yloc <= 17 && xloc >= 0 && xloc <= 30 {
                                    let nidx: usize = zz * 510 + yloc as usize * 30 + xloc as usize;
                                    if 0 != cache.board[nidx] {
                                        found = NONE_SELECTED;
                                    }
                                }
                            }
                        }
                    }

                    if NONE_SELECTED != found {
                        options.push(found - 1);
                    }
                }
            }
        }
    }

    let mut count: i32 = 0;

    for i in 0..options.len() {
        for j in (i + 1)..options.len() {
            if cache.stones[options[i]].suite == cache.stones[options[j]].suite && cache.stones[options[i]].number == cache.stones[options[j]].number {
                count = count + 1;
            }
        }
    }

    if 0 != options.len() && 0 == count {
        return true;
    }
    else {
        cache.options = count;
    }

    false
}


#[rustfmt::skip]
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let mut expect_blown = false;

    cache.frame = (cache.frame + 1) % 128;

    if !cache.ready {
        if cache.frame == 127 {
            cache.ready = true;
        }
        return false;
    }

    // Amortize workload
    if 0 == cache.frame && !cache.stuck {
        let op = cache.options;
        cache.stuck = check_stuck(cache);
        if !cache.stuck && op != cache.options {
            if 0 != cache.options {
                world.entity_set_text(1, format!("Options: {}", cache.options));
                let ln = world.text_get_width(1) as f32 * 0.5;
                world.entity_set_position_xy(1, 235.0 - ln, 10.0);
                expect_blown = true;
            } else {
                world.entity_set_text(1, format!("You Win!"));
                let ln = world.text_get_width(1) as f32 * 0.5;
                world.entity_set_position_xy(1, 235.0 - ln, 168.0);
                cache.win = true;
                expect_blown = true;
            }
        } else if cache.stuck {
            world.entity_set_text(1, format!("Stuck! Click to Shuffle."));
            let ln = world.text_get_width(1) as f32 * 0.5;
            world.entity_set_position_xy(1, 235.0 - ln, 10.0);        
            expect_blown = true;
        }

        world.entity_set_visibility(1, true);
    }
    else if 64 == cache.frame && !cache.stuck && !cache.win && cache.game_timer.floor() as i64 != cache.game_timer_last {
        let mut stimer: String = String::new();
        let mins: i32 = (cache.game_timer / 60.0).floor() as i32;
        if mins < 10 {
            stimer = format!("0");
        }
        stimer = format!("{}{}:", stimer, mins);
        let secs: i32 = (cache.game_timer - mins as f64 * 60.0).floor() as i32;
        if secs < 10 {
            stimer = format!("{}0", stimer);
        }
        stimer = format!("{}{}", stimer, secs);        

        world.entity_set_text(2, format!("{}", stimer));
        world.entity_set_position_xy(2, 235.0 - 17.0, 323.0);
        world.entity_set_visibility(2, true);
        cache.game_timer_last = cache.game_timer.floor() as i64;
        expect_blown = true;
    }

    cache.game_timer += 833.0e-6;
    
    expect_blown
}

#[rustfmt::skip]
pub fn event(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {

    if mgfw::EVENT_INPUT_MOUSE_BUTTON_UP != event_id { return false; }

    if cache.stuck || (world.mouse_x < 10 && world.mouse_y < 10) {
        // shuffle board
        for i in 0..BOARD_SZ {
            if 0 == cache.board[i] {
                continue;
            }

            let i0: usize = cache.board[i] - 1;
            if !cache.stones[i0].active {
                continue;
            }
    
            let mut i1: usize = i0;
            
            loop {
                let j: usize = (mgfw::rnd() * BOARD_SZ as f32).floor() as usize;
                if 0 != cache.board[j] {
                    i1 = cache.board[j] - 1;
                    break;
                }
            }
    
            let e0: usize = cache.stones[i0].entity;
            let e1: usize = cache.stones[i1].entity;

            let tsuite = cache.stones[i0].suite;
            let tnum = cache.stones[i0].number;
            cache.stones[i0].suite = cache.stones[i1].suite;
            cache.stones[i1].suite = tsuite;
            cache.stones[i0].number = cache.stones[i1].number;
            cache.stones[i1].number = tnum;

            let s0 = world.entity_get_billboard(e0);
            let s1 = world.entity_get_billboard(e1);
            world.entity_set_billboard(e0, s1);
            world.entity_set_billboard(e1, s0);
            
        }
        world.entity_set_visibility(1, false);
        cache.stuck = false;
        return false;
    }

    let mut found: usize = NONE_SELECTED;
    //println!("{},{}", world.mouse_x, world.mouse_y);
    for i in 0..NUM_STONES {
        let sidx = NUM_STONES - i - 1;
        if !cache.stones[sidx].active {
            continue;
        }
        let eidx = cache.stones[sidx].entity;
        let pos = world.entity_get_position(eidx);
        if world.mouse_x as f32 >= pos.x - 12.0 && world.mouse_x as f32 <= pos.x + 12.0 && world.mouse_y as f32 >= pos.y - 18.0 && world.mouse_y as f32 <= pos.y + 18.0 {
            //println!("Clicked on eidx {} sidx {}", eidx, sidx);
            found = sidx;
            break;
        }
    }

    // check constraints
    for z in 0..5 {
        for x in 0..30 {
            for y in 0..17 {
                let bidx: usize = z * 510 + y * 30 + x;
                if found + 1 == cache.board[bidx] {

                    // check lhs/rhs constraints
                    //println!("checking constraints on {}, {}", found, bidx);
                    let mut lhs: bool = false;
                    let mut rhs: bool = false;
                    for yy in 0..3 {
                        let yloc: i32 = y as i32 + yy as i32 - 1;
                        let xloc: i32 = x as i32 - 2;
                        if yloc >= 0 && yloc <= 17 && xloc >= 0 && xloc <= 30 {
                            let nidx: usize = z * 510 + yloc as usize * 30 + xloc as usize;
                            if 0 != cache.board[nidx] {
                                lhs = true;
                            }
                        }
                        let xloc: i32 = x as i32 + 2;
                        if yloc >= 0 && yloc <= 17 && xloc >= 0 && xloc <= 30 {
                            let nidx: usize = z * 510 + yloc as usize * 30 + xloc as usize;
                            if 0 != cache.board[nidx] {
                                rhs = true;
                            }
                        }
                    }
                    if lhs && rhs {
                        //println!("failed lr constraint");
                        found = NONE_SELECTED;
                        continue;
                    }

                    // check z constraint
                    for zz in (z + 1)..5 {
                        for xx in 0..3 {
                            for yy in 0..3 {
                                let yloc: i32 = y as i32 + yy as i32 - 1;
                                let xloc: i32 = x as i32 + xx as i32 - 1;
                                if yloc >= 0 && yloc <= 17 && xloc >= 0 && xloc <= 30 {
                                    let nidx: usize = zz * 510 + yloc as usize * 30 + xloc as usize;
                                    if 0 != cache.board[nidx] {
                                        //println!("failed z constraint");
                                        found = NONE_SELECTED;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if NONE_SELECTED == found {
        if NONE_SELECTED != cache.selected {
            world.entity_set_visibility(cache.stones[cache.selected].entity - 1, false);
            cache.selected = NONE_SELECTED;
        }
        return false;
    }

    if NONE_SELECTED == cache.selected {
        cache.selected = found;
        let eidx = cache.stones[cache.selected].entity;
        world.entity_set_visibility(eidx - 1, true);
    }
    else if found != cache.selected {
        if cache.stones[cache.selected].suite == cache.stones[found].suite && cache.stones[cache.selected].number == cache.stones[found].number {
            //println!("found pair! {}/{}, {}/{}", cache.selected, cache.stones[cache.selected].entity, found, cache.stones[found].entity);
            world.entity_set_visibility(cache.stones[cache.selected].entity - 1, false);
            world.entity_set_visibility(cache.stones[cache.selected].entity, false);
            world.entity_set_visibility(cache.stones[found].entity, false);
            //world.entity_set_alpha_ease(cache.stones[cache.selected].entity, 1.0, 0.0, 0.5); // to do - currently have an easing bug
            //world.entity_set_alpha_ease(cache.stones[found].entity, 1.0, 0.0, 0.5);
            for i in 0..BOARD_SZ {
                if cache.board[i] == cache.selected + 1 || cache.board[i] == found + 1 {
                    cache.board[i] = 0;
                }
            }
            cache.stones[cache.selected].active = false;
            cache.stones[found].active = false;
            cache.selected = NONE_SELECTED;
        }
        else {
            world.entity_set_visibility(cache.stones[cache.selected].entity - 1, false);
            cache.selected = found;
            world.entity_set_visibility(cache.stones[cache.selected].entity - 1, true);
        }
    }
    
    false
}

pub fn shutdown(_cache: &mut GameData, heap: &mut GameDataHeap) {
    // deallocate and overwrite existing memory
    *heap = GameDataHeap::default();
    
    // re-box and consume
    //let _temp = unsafe { Box::from_raw(cache.heap) };
}
