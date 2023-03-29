// For the Barnes-Hut acceleration calculation
pub const SOFTENING: f64 = 0.1;
pub const THETA: f64 = 0.9; // Used in acceleration calculation, the smaller the more accurate
pub const GRAVITY: f64 = 100.0;
pub const DT: f64 = 10.0; // Time step
pub const START_BOX_SIZE: f64 = 100.0; // The size of the box that contains all the bodies at the start
pub const SPAWNED_BODY_SPEED: f64 = 1.0 / 300.0; // The speed of the spawned bodies
pub const SPAWNED_BODY_SPEED_MOBILE: f64 = 1.0 / 600.0; // The speed of the spawned bodies

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
pub const BODY_DRAW_SIZE: i32 = 1;
pub const BODY_DRAW_SIZE_MOBILE: i32 = 3;
pub const SPAWN_BODY_DRAW_SIZE: i32 = 3;
pub const SPAWN_BODY_DRAW_SIZE_MOBILE: i32 = 5;
pub const SPAWN_BODY_COLOR: &str = "rgb(255,0,0)";
