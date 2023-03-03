use character::action::Actions;
use skill_system::Skill;
mod available_skills;
mod character;
mod skill_system;
fn main() {
    let skills = available_skills::generate_skills();
    skills
        .clone()
        .into_iter()
        .for_each(|skill| println!("{}", skill.name));
    let mut my_character = character::Pawn::new("Eric".to_string(), 100, 100, 12, 10, 10);

    let mut enemy = character::Pawn::new("Enemy".to_string(), 100, 100, 10, 5, 10);

    skills[0].learn(&mut my_character, 0);
    enemy.defend();
    loop {
        println!("Select an action:");
        println!("1. Attack");
        println!("2. Cast");
        println!("3. Defend");
        println!("4. Exit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                my_character.attack(&mut enemy);
            }
            "2" => {
                println!("Select a skill:");
                println!("1. Fireball");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                match input.trim() {
                    "1" => {
                        my_character.cast(&mut enemy, 0);
                    }
                    _ => {
                        println!("Invalid input!");
                    }
                }
            }
            "3" => {
                my_character.defend();
            }
            "4" => {
                break;
            }
            _ => {
                println!("Invalid input!");
            }
        }
        enemy.attack(&mut my_character);
        if enemy.health <= 0 {
            println!("You won!");
            break;
        }
    }
}
