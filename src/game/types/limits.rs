use super::value_types::{Capacity, FighterPower, ResourceValue};

// Set of constants that define our game values

// === BUILDING CAPACITIES ===
pub const BASE_CAPACITY: Capacity = 200;
// ===========================

// === ITEM COSTS ===
pub const BASE_COST: ResourceValue = (220, 100);
pub const ARCHER_COST: ResourceValue = (0, 10);
pub const WARRIOR_COST: ResourceValue = (10, 5);
// ==================

// === ACTION GAINS ===
pub const HARVEST_GAIN: ResourceValue = (200, 120);
// ====================

// === UNIT POWERS ====
pub const ARCHER_POWER: FighterPower = 1.9;
pub const WARRIOR_POWER: FighterPower = 1.2;
// ====================

// === DEFAULT GAME SIZE ====
pub const DEFAULT_PLAN_WIDTH: usize = 1;
pub const DEFAULT_PLAN_HEIGHT: usize = 1;
