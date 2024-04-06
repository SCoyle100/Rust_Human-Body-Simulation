mod human;
mod states;
mod indicators;
mod inputs;
mod systems;

use human::Human;
use inputs::ExternalInput;

fn main() {
    let mut simulated_human = Human::new();

    // Example of a basic simulation loop
    for day in 0..30 {
        println!("Day: {}", day);
        
        // Example of applying daily inputs
        simulated_human.apply_input(ExternalInput::Sunlight(8));
        simulated_human.apply_input(ExternalInput::Food(inputs::FoodType::Healthy));
        
        // Simulate a step (e.g., a day in the simulation)
        simulated_human.simulate_step();

        // Optionally, print out current state of the human
        println!("Current State: {:?}", simulated_human.current_state());
        println!("Physiological Indicators: {:?}", simulated_human.indicators());
    }
}

