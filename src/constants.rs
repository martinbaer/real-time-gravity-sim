// For the Barnes-Hut acceleration calculation
pub const SOFTENING: f64 = 0.1;
pub const THETA: f64 = 0.9; // Used in acceleration calculation, the smaller the more accurate
pub const BODIES_PER_SPAWN: usize = 2; // The number of bodies to spawn at a time
pub const START_BOX_SIZE: f64 = 50.0; // The size of the box that contains all the bodies at the start

// For the Barnes-Hut tree construction
pub const TREE_GROWTH_INCREMENT: usize = 100;
pub const ROOT_NODE_INDEX: usize = 0;

// For drawing bodies on canvas
pub const STAR_COLOURS_LEN: usize = 15;
pub const STAR_COLOURS: [&str; STAR_COLOURS_LEN] = [
    "rgb(155,176,255)",
    "rgb(170,191,255)",
    "rgb(202,215,255)",
    "rgb(248,247,255)",
    "rgb(248,247,255)",
    "rgb(248,247,255)",
    "rgb(255,255,255)",
    "rgb(248,247,255)",
    "rgb(255,255,255)",
    "rgb(255,255,255)",
    "rgb(255,255,255)",
    "rgb(255,244,234)",
    "rgb(255,244,234)",
    "rgb(255,210,161)",
    "rgb(255,204,111)",
];
pub const BODY_DRAW_SIZE: i32 = 2;
pub const BODY_DRAW_SIZE_MOBILE: i32 = 3;
pub const SPAWN_BODY_DRAW_SIZE: i32 = 3;
pub const SPAWN_BODY_DRAW_SIZE_MOBILE: i32 = 20;
pub const SPAWN_BODY_COLOR: &str = "rgb(255,0,0)";
