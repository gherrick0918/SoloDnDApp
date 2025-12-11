use solo_engine::campaign::Campaign;
use solo_engine::engine::Engine;
use solo_engine::rules::Character;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load JSON files from ../content (relative to the engine crate dir).
    let campaign_json = std::fs::read_to_string("../content/campaigns/road_to_redcrest.json")?;
    let character_json = std::fs::read_to_string("../content/characters/pregen_fighter.json")?;

    let campaign: Campaign = Campaign::from_json(&campaign_json)?;
    let character: Character = Character::from_json(&character_json)?;

    let mut engine = Engine::new(campaign, character, 42);

    loop {
        let view = engine.current_view();

        if let Some(title) = &view.title {
            println!("\n== {} ==", title);
        }
        for para in &view.text {
            if !para.is_empty() {
                println!("{}", para);
            }
        }
        println!(
            "\n{} (Lv {}) HP {}/{}",
            view.character_summary.name,
            view.character_summary.level,
            view.character_summary.current_hp,
            view.character_summary.max_hp
        );

        if view.choices.is_empty() {
            println!("\n[END OF ADVENTURE]");
            break;
        }

        println!("\nChoices:");
        for (i, c) in view.choices.iter().enumerate() {
            println!("  {}. {}", i + 1, c.label);
        }
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let trimmed = line.trim();

        let idx: usize = match trimmed.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        if idx == 0 || idx > view.choices.len() {
            println!("Choice out of range.");
            continue;
        }

        let choice = &view.choices[idx - 1];
        engine.choose(&choice.id);
    }

    Ok(())
}
