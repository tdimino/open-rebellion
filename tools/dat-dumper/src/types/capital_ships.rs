use crate::codec::{ByteReader, ByteWriter};
use crate::dat_record::DatRecord;
use serde::Serialize;

// CAPSHPSD.DAT — 30 capital ships, 200 bytes per record
// File size: 16 (header) + 30 * 200 = 6016 bytes
// Header: field1=1, count=30, family_id=20, field4=28

#[derive(Debug, Clone, Serialize)]
pub struct CapitalShipsFile {
    pub field1: u32,
    pub count: u32,
    pub family_id: u32,
    pub field4: u32,
    pub ships: Vec<CapitalShip>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CapitalShip {
    pub id: u32,
    pub field2: u32,
    pub production_family: u32,
    pub next_production_family: u32,
    pub family_id: u32,
    pub text_stra_dll_id: u16,
    pub field7: u16,
    pub is_alliance: u32,
    pub is_empire: u32,
    pub refined_material_cost: u32,
    pub maintenance_cost: u32,
    pub research_order: u32,
    pub research_difficulty: u32,
    pub uprising_defense: u32,
    pub detection: u32,
    pub shield_strength: u32,
    pub sub_light_engine: u32,
    pub maneuverability: u32,
    pub hyperdrive: u32,
    pub hyperdrive_if_damaged: u32,
    pub turbolaser_fore: u32,
    pub ion_cannon_fore: u32,
    pub laser_cannon_fore: u32,
    pub turbolaser_aft: u32,
    pub ion_cannon_aft: u32,
    pub laser_cannon_aft: u32,
    pub turbolaser_port: u32,
    pub ion_cannon_port: u32,
    pub laser_cannon_port: u32,
    pub turbolaser_starboard: u32,
    pub ion_cannon_starboard: u32,
    pub laser_cannon_starboard: u32,
    pub turbolaser_range: u32,
    pub ion_cannon_range: u32,
    pub laser_cannon_range: u32,
    pub turbolaser_attack_strength: u32,
    pub ion_cannon_attack_strength: u32,
    pub laser_cannon_attack_strength: u32,
    pub overall_attack_strength: u32,
    pub hull: u32,
    pub tractor_beam_power: u32,
    pub tractor_beam_range: u32,
    pub gravity_well_projector: u32,
    pub interdiction_strength: u32,
    pub bombardment_modifier: u32,
    pub damage_control: u32,
    pub weapon_recharge_rate: u32,
    pub shield_recharge_rate: u32,
    pub fighter_capacity: u32,
    pub troop_capacity: u32,
    pub field51: u32,
}

impl DatRecord for CapitalShipsFile {
    fn parse(r: &mut ByteReader) -> anyhow::Result<Self> {
        let field1 = r.read_u32()?;
        let count = r.read_u32()?;
        let family_id = r.read_u32()?;
        let field4 = r.read_u32()?;
        let mut ships = Vec::with_capacity(count as usize);
        for _ in 0..count {
            ships.push(CapitalShip::parse_entry(r)?);
        }
        Ok(Self { field1, count, family_id, field4, ships })
    }

    fn write_bytes(&self, w: &mut ByteWriter) {
        w.write_u32(self.field1);
        w.write_u32(self.count);
        w.write_u32(self.family_id);
        w.write_u32(self.field4);
        for ship in &self.ships {
            ship.write_entry(w);
        }
    }
}

impl CapitalShip {
    fn parse_entry(r: &mut ByteReader) -> anyhow::Result<Self> {
        Ok(Self {
            id: r.read_u32()?,
            field2: r.read_u32()?,
            production_family: r.read_u32()?,
            next_production_family: r.read_u32()?,
            family_id: r.read_u32()?,
            text_stra_dll_id: r.read_u16()?,
            field7: r.read_u16()?,
            is_alliance: r.read_u32()?,
            is_empire: r.read_u32()?,
            refined_material_cost: r.read_u32()?,
            maintenance_cost: r.read_u32()?,
            research_order: r.read_u32()?,
            research_difficulty: r.read_u32()?,
            uprising_defense: r.read_u32()?,
            detection: r.read_u32()?,
            shield_strength: r.read_u32()?,
            sub_light_engine: r.read_u32()?,
            maneuverability: r.read_u32()?,
            hyperdrive: r.read_u32()?,
            hyperdrive_if_damaged: r.read_u32()?,
            turbolaser_fore: r.read_u32()?,
            ion_cannon_fore: r.read_u32()?,
            laser_cannon_fore: r.read_u32()?,
            turbolaser_aft: r.read_u32()?,
            ion_cannon_aft: r.read_u32()?,
            laser_cannon_aft: r.read_u32()?,
            turbolaser_port: r.read_u32()?,
            ion_cannon_port: r.read_u32()?,
            laser_cannon_port: r.read_u32()?,
            turbolaser_starboard: r.read_u32()?,
            ion_cannon_starboard: r.read_u32()?,
            laser_cannon_starboard: r.read_u32()?,
            turbolaser_range: r.read_u32()?,
            ion_cannon_range: r.read_u32()?,
            laser_cannon_range: r.read_u32()?,
            turbolaser_attack_strength: r.read_u32()?,
            ion_cannon_attack_strength: r.read_u32()?,
            laser_cannon_attack_strength: r.read_u32()?,
            overall_attack_strength: r.read_u32()?,
            hull: r.read_u32()?,
            tractor_beam_power: r.read_u32()?,
            tractor_beam_range: r.read_u32()?,
            gravity_well_projector: r.read_u32()?,
            interdiction_strength: r.read_u32()?,
            bombardment_modifier: r.read_u32()?,
            damage_control: r.read_u32()?,
            weapon_recharge_rate: r.read_u32()?,
            shield_recharge_rate: r.read_u32()?,
            fighter_capacity: r.read_u32()?,
            troop_capacity: r.read_u32()?,
            field51: r.read_u32()?,
        })
    }

    fn write_entry(&self, w: &mut ByteWriter) {
        w.write_u32(self.id);
        w.write_u32(self.field2);
        w.write_u32(self.production_family);
        w.write_u32(self.next_production_family);
        w.write_u32(self.family_id);
        w.write_u16(self.text_stra_dll_id);
        w.write_u16(self.field7);
        w.write_u32(self.is_alliance);
        w.write_u32(self.is_empire);
        w.write_u32(self.refined_material_cost);
        w.write_u32(self.maintenance_cost);
        w.write_u32(self.research_order);
        w.write_u32(self.research_difficulty);
        w.write_u32(self.uprising_defense);
        w.write_u32(self.detection);
        w.write_u32(self.shield_strength);
        w.write_u32(self.sub_light_engine);
        w.write_u32(self.maneuverability);
        w.write_u32(self.hyperdrive);
        w.write_u32(self.hyperdrive_if_damaged);
        w.write_u32(self.turbolaser_fore);
        w.write_u32(self.ion_cannon_fore);
        w.write_u32(self.laser_cannon_fore);
        w.write_u32(self.turbolaser_aft);
        w.write_u32(self.ion_cannon_aft);
        w.write_u32(self.laser_cannon_aft);
        w.write_u32(self.turbolaser_port);
        w.write_u32(self.ion_cannon_port);
        w.write_u32(self.laser_cannon_port);
        w.write_u32(self.turbolaser_starboard);
        w.write_u32(self.ion_cannon_starboard);
        w.write_u32(self.laser_cannon_starboard);
        w.write_u32(self.turbolaser_range);
        w.write_u32(self.ion_cannon_range);
        w.write_u32(self.laser_cannon_range);
        w.write_u32(self.turbolaser_attack_strength);
        w.write_u32(self.ion_cannon_attack_strength);
        w.write_u32(self.laser_cannon_attack_strength);
        w.write_u32(self.overall_attack_strength);
        w.write_u32(self.hull);
        w.write_u32(self.tractor_beam_power);
        w.write_u32(self.tractor_beam_range);
        w.write_u32(self.gravity_well_projector);
        w.write_u32(self.interdiction_strength);
        w.write_u32(self.bombardment_modifier);
        w.write_u32(self.damage_control);
        w.write_u32(self.weapon_recharge_rate);
        w.write_u32(self.shield_recharge_rate);
        w.write_u32(self.fighter_capacity);
        w.write_u32(self.troop_capacity);
        w.write_u32(self.field51);
    }
}
