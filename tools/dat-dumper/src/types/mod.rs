// Pattern 1 — entity tables
pub mod capital_ships;
pub mod fighters;
pub mod troops;
pub mod special_forces;
pub mod major_characters;
pub mod minor_characters;
pub mod systems;
pub mod sectors;
pub mod defense_facilities;
pub mod manufacturing_facilities;
pub mod production_facilities;

// Pattern 2 — parameter tables
pub mod general_params;
pub mod side_params;

// Pattern 3 — seed tables (generic, shared by all CMUN*/FACL* files)
pub mod seed_table;
