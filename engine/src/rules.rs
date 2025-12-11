use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbilityScores {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    #[serde(rename = "int")]
    pub int_: i32,
    pub wis: i32,
    pub cha: i32,
}

impl AbilityScores {
    pub fn modifier(&self, ability: Ability) -> i32 {
        let score = match ability {
            Ability::Strength => self.str,
            Ability::Dexterity => self.dex,
            Ability::Constitution => self.con,
            Ability::Intelligence => self.int_,
            Ability::Wisdom => self.wis,
            Ability::Charisma => self.cha,
        };
        (score - 10) / 2
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub name: String,
    pub level: u8,
    pub abilities: AbilityScores,
    pub max_hp: i32,
    pub current_hp: i32,
    pub ac: i32,
    pub proficiency_bonus: i32,
}

impl Character {
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

pub fn ability_from_str(s: &str) -> Option<Ability> {
    match s.to_ascii_uppercase().as_str() {
        "STR" | "STRENGTH" => Some(Ability::Strength),
        "DEX" | "DEXTERITY" => Some(Ability::Dexterity),
        "CON" | "CONSTITUTION" => Some(Ability::Constitution),
        "INT" | "INTELLIGENCE" => Some(Ability::Intelligence),
        "WIS" | "WISDOM" => Some(Ability::Wisdom),
        "CHA" | "CHARISMA" => Some(Ability::Charisma),
        _ => None,
    }
}
