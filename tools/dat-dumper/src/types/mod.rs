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

// Pattern 1 — new entity tables
pub mod missions;
pub mod fleets_seed;
pub mod entity_table;
pub mod all_facilities;

// Pattern 2 — parameter tables
pub mod general_params;
pub mod side_params;

// Pattern 2 — int lookup tables (shared by all *MSTB and gameplay tables)
pub mod int_table;

// Pattern 2 — system facility seed tables (SYFCCRTB, SYFCRMTB)
pub mod syfc_table;

// Pattern 3 — seed tables (generic, shared by all CMUN*/FACL* files)
pub mod seed_table;

// Win32 PE resource extraction
pub mod textstra;
