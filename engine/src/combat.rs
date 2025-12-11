use crate::dice::DiceRoller;
use crate::rules::{Ability, Character};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Monster {
    pub name: String,
    pub ac: i32,
    pub max_hp: i32,
    pub current_hp: i32,
    pub attack_bonus: i32,
    pub damage_dice_count: u8,
    pub damage_dice_sides: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Encounter {
    pub monsters: Vec<Monster>,
    pub in_progress: bool,
}

impl Encounter {
    pub fn is_over(&self, hero: &Character) -> bool {
        !self.in_progress || hero.current_hp <= 0 || self.monsters.iter().all(|m| m.current_hp <= 0)
    }

    pub fn first_alive_monster_mut(&mut self) -> Option<&mut Monster> {
        self.monsters.iter_mut().find(|m| m.current_hp > 0)
    }
}

pub fn hero_attack(hero: &Character, monster: &mut Monster, dice: &mut DiceRoller) -> String {
    let roll = dice.d20();
    let attack_bonus = hero.abilities.modifier(Ability::Strength) + hero.proficiency_bonus;
    let total = roll + attack_bonus;

    if total >= monster.ac {
        let dmg_roll = dice.roll(1, 8);
        let dmg = (dmg_roll + hero.abilities.modifier(Ability::Strength)).max(1);
        monster.current_hp -= dmg;
        format!("You hit {} for {} damage!", monster.name, dmg)
    } else {
        format!("You miss {}.", monster.name)
    }
}

pub fn monster_attack(monster: &Monster, hero: &mut Character, dice: &mut DiceRoller) -> String {
    let roll = dice.d20();
    let total = roll + monster.attack_bonus;
    if total >= hero.ac {
        let dmg = dice.roll(monster.damage_dice_count, monster.damage_dice_sides);
        hero.current_hp -= dmg.max(1);
        format!("{} hits you for {} damage!", monster.name, dmg.max(1))
    } else {
        format!("{} misses you.", monster.name)
    }
}
