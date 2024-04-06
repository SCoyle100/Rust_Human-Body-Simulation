use crate::indicators::HealthIndicators;
use crate::ExternalInput;



// Ensure other necessary imports are here

pub struct Human {
    indicators: HealthIndicators,
    current_state: MainState,
}

#[derive(Debug)] 
pub enum MainState {
    Awake,
    Sleep,
    // Extend with more detailed states
}

impl Human {
    pub fn new() -> Self {
        Human {
            indicators: HealthIndicators::new(), // Use the new HealthIndicators struct
            current_state: MainState::Awake, // Default state remains unchanged
        }
    }

    pub fn apply_input(&mut self, input: ExternalInput) {
        // Implementation will vary based on how inputs affect the human
        // For example, adjusting blood sugar based on food intake
    }

    pub fn simulate_step(&mut self) {
        // Simulate changes to the human's state and indicators over a time step
        // For example, decrease blood sugar gradually
    }

    // Utility methods to access the human's state and indicators
    pub fn current_state(&self) -> &MainState {
        &self.current_state
    }

    pub fn indicators(&self) -> &HealthIndicators {
        &self.indicators
    }
}


