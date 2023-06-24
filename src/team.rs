pub mod team{
    use std::fmt; // Import `fmt`

    use crate::random_engine::rng_eng::{
        AttributeTypes,
        RollType,
        RollResult,
    };

    use crate::player::player::{
        FORWARDS,
        Player,
        get_position,
    };


    // Team struct
    pub struct Team {
        pub name : String,
        pub score : i32,
        pub players : Vec<Player>,
    }

    impl Team{
        // New
        // Default empty Team
        pub fn new() -> Team{
            let mut tmp = Team{
                name : "".to_string(),
                score : 0,
                // Create and vector of random players
                players : (0..26).map(|_| Player::new()).collect::<Vec <Player>>(),
            };

            for i in 0..tmp.players.len(){
                tmp.players[i].position.push(
                    get_position((i+1)
                    .try_into().unwrap())
                );
                if i < 15 {
                    tmp.players[i].selected_position = get_position((i+1).try_into().unwrap());
                }
            }

            return tmp;
        }

        // Team Challange Roll
        // Group challange roll for the entire team
        pub fn challange_roll(&self, attr : &AttributeTypes) -> i32 {
            
            self.players
                .iter()                             // For each player
                .filter( |x| x.is_selected)         // filter by active players
                .map(|x| x.challange_roll(*attr).0)   // Roll an individual challange roll
                .sum()                              // Sum the team challange roll
        }

        // Forwards Challanage
        pub fn forwards_challange_roll(&self, attr : &AttributeTypes) -> i32{
            
            self.players
                .iter()                                     // For each player
                .filter( |x| x.is_selected 
                    && FORWARDS.contains(&x.position[0]))   // filter by active players and forwards
                .map(|x| x.challange_roll(*attr).0)           // Roll an individual challange roll
                .sum()                                      // Sum the team challange roll
        }


        // Backs Challanage
        pub fn backs_challange_roll(&self, attr : &AttributeTypes) -> i32{
        
            self.players
                .iter()                                     // For each player
                .filter( |x| x.is_selected 
                    && !FORWARDS.contains(&x.position[0]))  // filter by active players and backs
                .map(|x| x.challange_roll(*attr).0)           // Roll an individual challange roll
                .sum()                                      // Sum the team challange roll
        }

    }
}
