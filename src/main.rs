mod human;
mod states;
mod indicators;
mod inputs;
mod systems;

use human::Human;
use std::io::{self, Write};

use crate::inputs::ExternalInput;

fn main() {
    let mut simulated_human = Human::new();

    for day in 0..30 {
        println!("Day: {}", day);

        // Prompt for and read sunlight hours
        print!("Enter sunlight hours for day {}: ", day);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        let mut sunlight_hours_str = String::new();
        io::stdin().read_line(&mut sunlight_hours_str).unwrap();
        let sunlight_hours: u8 = sunlight_hours_str.trim().parse().expect("Please type a number!");

        // Prompt for and read food type
        print!("Enter food type for day {} (Healthy/Unhealthy): ", day);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately
        let mut food_type_str = String::new();
        io::stdin().read_line(&mut food_type_str).unwrap();
        let food_type = match food_type_str.trim() {
            "Healthy" => inputs::FoodType::Healthy,
            "Unhealthy" => inputs::FoodType::Unhealthy,
            _ => panic!("Invalid food type! Use 'Healthy' or 'Unhealthy'."),
        };

        // Apply daily inputs
        simulated_human.apply_input(ExternalInput::Sunlight(sunlight_hours));
        simulated_human.apply_input(ExternalInput::Food(food_type));

        // Simulate a step (e.g., a day in the simulation)
        simulated_human.simulate_step();

        println!("Current State: {:?}", simulated_human.current_state());
        println!("Physiological Indicators: {:?}", simulated_human.indicators());
    }
}

