pub enum FoodType {
    Healthy,
    Unhealthy,
    
}

pub enum ExternalInput {
    Sunlight(u8), // Hours of sunlight
    Food(FoodType),
}

impl ExternalInput {
    // Example of a utility function to create a new Food input
    pub fn new_food(food_type: FoodType) -> Self {
        ExternalInput::Food(food_type)
    }

    // Example of a utility function to create a new Sunlight input
    pub fn new_sunlight(hours: u8) -> Self {
        ExternalInput::Sunlight(hours)
    }
}

