use crate::combat::{hero_attack, monster_attack, Encounter, Monster};
use crate::dice::DiceRoller;
use crate::rules::{ability_from_str, Character};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Narrative,
    Combat,
    End,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SkillCheck {
    pub ability: String,
    pub dc: i32,
    #[serde(default)]
    pub success_next: Option<String>,
    #[serde(default)]
    pub failure_next: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Choice {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(default)]
    pub skill_check: Option<SkillCheck>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MonsterSpec {
    pub r#ref: String,
    pub count: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EncounterSpec {
    pub monsters: Vec<MonsterSpec>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: NodeType,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub text: Vec<String>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub encounter: Option<EncounterSpec>,
    #[serde(default)]
    pub on_victory: Option<String>,
    #[serde(default)]
    pub on_defeat: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub character: Character,
    pub current_node_id: String,
    pub encounter: Option<Encounter>,
    pub last_log: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceView {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterSummary {
    pub name: String,
    pub level: u8,
    pub current_hp: i32,
    pub max_hp: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeView {
    pub title: Option<String>,
    pub text: Vec<String>,
    pub choices: Vec<ChoiceView>,
    pub character_summary: CharacterSummary,
    #[serde(default)]
    pub log: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Campaign {
    pub id: String,
    pub title: String,
    #[serde(rename = "startNodeId")]
    pub start_node_id: String,
    pub nodes: Vec<Node>,
}

impl Campaign {
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    fn find_node(&self, id: &str) -> &Node {
        self.nodes
            .iter()
            .find(|n| n.id == id)
            .unwrap_or_else(|| panic!("Node not found: {}", id))
    }

    pub fn view_node(&self, state: &GameState) -> NodeView {
        let node = self.find_node(&state.current_node_id);
        let choices = match node.kind {
            NodeType::End => Vec::new(),
            _ => node
                .choices
                .iter()
                .map(|c| ChoiceView {
                    id: c.id.clone(),
                    label: c.label.clone(),
                })
                .collect(),
        };

        let char_sum = CharacterSummary {
            name: state.character.name.clone(),
            level: state.character.level,
            current_hp: state.character.current_hp,
            max_hp: state.character.max_hp,
        };

        let mut text = node.text.clone();
        if let Some(log) = &state.last_log {
            if !log.is_empty() {
                text.push(String::new());
                text.push(log.clone());
            }
        }

        NodeView {
            title: node.title.clone(),
            text,
            choices,
            character_summary: char_sum,
            log: state.last_log.clone(),
        }
    }

    fn build_encounter(&self, node: &Node) -> Option<Encounter> {
        let spec = node.encounter.as_ref()?;
        let mut monsters = Vec::new();
        for m in &spec.monsters {
            for _ in 0..m.count {
                monsters.push(make_monster_from_ref(&m.r#ref));
            }
        }
        Some(Encounter {
            monsters,
            in_progress: true,
        })
    }

    pub fn apply_choice(&self, choice_id: &str, state: &mut GameState, dice: &mut DiceRoller) {
        let node = self.find_node(&state.current_node_id);
        match node.kind {
            NodeType::Narrative => self.apply_narrative_choice(node, choice_id, state, dice),
            NodeType::Combat => self.apply_combat_choice(node, choice_id, state, dice),
            NodeType::End => {
                state.last_log = Some("The adventure is over.".to_string());
            }
        }
    }

    fn apply_narrative_choice(
        &self,
        node: &Node,
        choice_id: &str,
        state: &mut GameState,
        dice: &mut DiceRoller,
    ) {
        let choice = match node.choices.iter().find(|c| c.id == choice_id) {
            Some(c) => c,
            None => {
                state.last_log = Some(format!("Unknown choice: {}", choice_id));
                return;
            }
        };

        if let Some(sc) = &choice.skill_check {
            let ability = match ability_from_str(&sc.ability) {
                Some(a) => a,
                None => {
                    state.last_log =
                        Some(format!("Unknown ability in skill check: {}", sc.ability));
                    return;
                }
            };
            let roll = dice.d20();
            let modif = state.character.abilities.modifier(ability);
            let total = roll + modif;
            let success = total >= sc.dc;

            let mut log = format!(
                "Skill check ({}, DC {}): rolled {} + {} = {} => {}",
                sc.ability,
                sc.dc,
                roll,
                modif,
                total,
                if success { "success" } else { "failure" }
            );
            if let Some(desc) = &sc.description {
                log = format!(
                    "{}
{}",
                    desc, log
                );
            }
            state.last_log = Some(log);

            let next_id = if success {
                sc.success_next.clone()
            } else {
                sc.failure_next.clone()
            };

            if let Some(next) = next_id {
                state.current_node_id = next;
            }
            return;
        }

        if let Some(next) = &choice.next {
            state.current_node_id = next.clone();
            state.last_log = None;
        } else {
            state.last_log = Some("Nowhere to go from here.".to_string());
        }
    }

    fn apply_combat_choice(
        &self,
        node: &Node,
        choice_id: &str,
        state: &mut GameState,
        dice: &mut DiceRoller,
    ) {
        if state.encounter.is_none() {
            state.encounter = self.build_encounter(node);
        }

        let encounter = match &mut state.encounter {
            Some(e) => e,
            None => {
                state.last_log = Some("No encounter to resolve.".to_string());
                return;
            }
        };

        let mut log_lines = Vec::new();

        match choice_id {
            "attack" => {
                if let Some(monster) = encounter.first_alive_monster_mut() {
                    log_lines.push(hero_attack(&state.character, monster, dice));
                } else {
                    log_lines.push("There is nothing left to attack.".to_string());
                }
            }
            "continue" => {
                log_lines.push("You press on...".to_string());
            }
            _ => {
                log_lines.push(format!("Unknown combat choice: {}", choice_id));
            }
        }

        if state.character.current_hp > 0 {
            for monster in encounter.monsters.iter() {
                if monster.current_hp > 0 && state.character.current_hp > 0 {
                    log_lines.push(monster_attack(monster, &mut state.character, dice));
                }
            }
        }

        if encounter.is_over(&state.character) {
            encounter.in_progress = false;
            if state.character.current_hp > 0 {
                if let Some(next) = &node.on_victory {
                    log_lines.push("You won the fight!".to_string());
                    state.current_node_id = next.clone();
                } else {
                    log_lines.push("You have won, but the story has nowhere to go.".to_string());
                }
            } else if let Some(next) = &node.on_defeat {
                log_lines.push("You have been defeated...".to_string());
                state.current_node_id = next.clone();
            }
        }

        state.last_log = Some(log_lines.join("\n"));
    }
}

fn make_monster_from_ref(r: &str) -> Monster {
    match r {
        "srd_goblin" => Monster {
            name: "Goblin".to_string(),
            ac: 15,
            max_hp: 7,
            current_hp: 7,
            attack_bonus: 4,
            damage_dice_count: 1,
            damage_dice_sides: 6,
        },
        _ => Monster {
            name: r.to_string(),
            ac: 12,
            max_hp: 8,
            current_hp: 8,
            attack_bonus: 3,
            damage_dice_count: 1,
            damage_dice_sides: 6,
        },
    }
}
