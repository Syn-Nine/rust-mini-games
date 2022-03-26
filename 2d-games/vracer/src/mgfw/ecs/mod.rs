pub mod component_angle;
pub mod component_color;
pub mod component_easing;
pub mod component_frame;
pub mod component_physics;
pub mod component_position;
pub mod component_render;
pub mod component_render_billboard;
pub mod component_render_line_buffer;
pub mod component_render_text;
pub mod component_render_tilemap;
pub mod component_render_triangle_buffer;
pub mod component_scale;
pub mod component_projection;
pub mod entity;
pub mod system_easing;
pub mod system_physics;
pub mod system_render;
pub mod world;

use entity::*;
pub use world::*;

pub use component_angle::*;
pub use component_color::*;
use component_easing::*;
use component_frame::*;
pub use component_physics::*;
pub use component_position::*;
pub use component_projection::*;
use component_render::*;
use component_render_billboard::*;
use component_render_line_buffer::*;
use component_render_text::*;
pub use component_render_tilemap::*;
use component_render_triangle_buffer::*;
pub use component_scale::*;

pub use system_easing::*;
pub use system_physics::*;
pub use system_render::*;

use super::cache::CacheManager;
use super::fonts;
use super::support::Gl;

pub const COMPONENT_ACTIVE: u32 = 1 << 0;
pub const COMPONENT_POSITION: u32 = 1 << 1;
pub const COMPONENT_RENDER: u32 = 1 << 2;
pub const COMPONENT_VISIBLE: u32 = 1 << 3;
pub const COMPONENT_PHYSICS: u32 = 1 << 4;
pub const COMPONENT_ANGLE: u32 = 1 << 5;
pub const COMPONENT_SCALE: u32 = 1 << 6;
pub const COMPONENT_COLOR: u32 = 1 << 7;
pub const COMPONENT_FRAME: u32 = 1 << 8;
pub const COMPONENT_PROJECTION: u32 = 1 << 9;

pub const EMPTY_TILE: u16 = 0;
