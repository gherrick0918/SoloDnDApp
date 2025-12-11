use crate::campaign::{Campaign, GameState, NodeView};
use crate::dice::DiceRoller;
use crate::rules::Character;

pub struct Engine {
    pub campaign: Campaign,
    pub state: GameState,
    dice: DiceRoller,
}

impl Engine {
    pub fn new(campaign: Campaign, character: Character, seed: u64) -> Self {
        let start_id = campaign.start_node_id.clone();
        Self {
            campaign,
            state: GameState {
                character,
                current_node_id: start_id,
                encounter: None,
                last_log: None,
            },
            dice: DiceRoller::new(seed),
        }
    }

    pub fn current_view(&self) -> NodeView {
        self.campaign.view_node(&self.state)
    }

    pub fn choose(&mut self, choice_id: &str) {
        self.campaign
            .apply_choice(choice_id, &mut self.state, &mut self.dice);
    }
}
