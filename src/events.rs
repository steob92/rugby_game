// Module to handle the match events

pub mod events {
    use crate::player::player::Player;
    use crate::random_engine::rng_eng::{AttributeTypes, RollResult};
    use crate::team::team::Team;

    // Define the contested and team checks first

    // tackle
    // Return a bool and the roll result
    pub fn tackle(ball_carrier: &Player, tackler: &Player) -> (bool, RollResult) {
        // Ball carrier will make a dex check
        let (bc_score, bc_result) = ball_carrier.challange_roll(AttributeTypes::Dexterity);
        // Tackler will make a strength check
        let (tk_score, tk_result) = tackler.challange_roll(AttributeTypes::Strength);

        match (bc_result, tk_result) {
            // Ball carrier 20, Tackler 2-19
            (RollResult::CriticalSuccess, RollResult::Flat) => (false, RollResult::CriticalSuccess),
            // Ball carrier 2-19, Tackler 20
            (RollResult::Flat, RollResult::CriticalSuccess) => (true, RollResult::CriticalSuccess),
            // Ball carrier 1, Tackler 2-19
            (RollResult::CriticalFail, RollResult::Flat) => (false, RollResult::CriticalFail),
            // Ball carrier 2-19, Tackler 1
            (RollResult::Flat, RollResult::CriticalFail) => (true, RollResult::CriticalFail),
            // Ball carrier 1, Tackler 1
            (RollResult::CriticalFail, RollResult::CriticalFail) => {
                (bc_score < tk_score, RollResult::CriticalFail)
            }
            // Ball carrier 20, Tackler 20
            (RollResult::CriticalSuccess, RollResult::CriticalSuccess) => {
                (bc_score < tk_score, RollResult::CriticalSuccess)
            }
            // Ball carrier 2-19, Tackler 2-19
            (RollResult::Flat, RollResult::Flat) => (bc_score < tk_score, RollResult::Flat),
            // Ball carrier 20, tackler 1
            (RollResult::CriticalSuccess, RollResult::CriticalFail) => {
                (false, RollResult::CriticalSuccess)
            }
            // Ball carrier 1, tackler 20
            (RollResult::CriticalFail, RollResult::CriticalSuccess) => {
                (true, RollResult::CriticalSuccess)
            }
        }
    }

    // Scrum
    // This is a c
}
