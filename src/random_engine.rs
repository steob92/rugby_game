// use rand::distributions::{Distribution, Uniform};

// Random engine for the game
pub mod rng_eng {

    use crate::com::{get_max, get_min};
    use rand::distributions::{Distribution, Uniform};

    // Maybe usefull later when actually talking about rolls
    #[allow(dead_code)]
    pub enum RollType {
        Flat,
        Advantage,
        Disavantage,
    }

    // Roll result, allowing for special events based on the roll result
    #[derive(Debug)]
    pub enum RollResult {
        CriticalFail,    // roll 1
        CriticalSuccess, // roll 20
        Flat,
    }

    // Flat player like roll
    pub fn roll(attr: i8, roll: RollType) -> (i32, RollResult) {
        // Draw a random d20
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..21);

        // Flat 1 roll
        // Advantage max of 2 rolls
        // Disadvantage min of 2 rolls
        let dice = match roll {
            RollType::Flat => die.sample(&mut rng),
            RollType::Advantage => get_max(die.sample(&mut rng), die.sample(&mut rng)),
            RollType::Disavantage => get_min(die.sample(&mut rng), die.sample(&mut rng)),
        };

        // println!("Dice: {}, Skill {}", dice, attr / 2);
        // Add the attibute
        let roll_result = match dice {
            1 => RollResult::CriticalFail,
            20 => RollResult::CriticalSuccess,
            _ => RollResult::Flat,
        };

        ((dice + attr / 2).into(), roll_result)
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy, PartialEq)]
    // #[derive(PartialEq)]
    pub enum AttributeTypes {
        Strength,
        Constitution,
        Dexterity,
        Intelligence,
        Wisdom,
        Charisma,
    }

    // Attributes
    // For the moment I'll use dnd-like attributes
    #[allow(dead_code)]
    #[derive(Clone)]
    pub struct Attributes {
        stre: i8,
        cons: i8,
        dext: i8,
        inte: i8,
        wisd: i8,
        chrm: i8,
    }

    // Implementation block, all `Attributes` associated functions & methods go in here
    impl Attributes {
        // Another associated function, taking two arguments:
        pub fn new() -> Attributes {
            Attributes {
                stre: 0,
                cons: 0,
                dext: 0,
                inte: 0,
                wisd: 0,
                chrm: 0,
            }
        }

        // Implement a Challange Roll
        pub fn challange_roll(
            &self,
            att_type: AttributeTypes,
            roll_type: RollType,
        ) -> (i32, RollResult) {
            let att = match att_type {
                AttributeTypes::Strength => self.stre,
                AttributeTypes::Constitution => self.cons,
                AttributeTypes::Dexterity => self.dext,
                AttributeTypes::Intelligence => self.inte,
                AttributeTypes::Wisdom => self.wisd,
                AttributeTypes::Charisma => self.chrm,
            };
            roll(att, roll_type)
        }
    }
}

// Random name generator
pub mod rnd_name {
    use rand::distributions::{Distribution, Uniform};
    // use rand::ThreadRng

    // Struct to control the rnd name generator
    pub struct NameGenerator {
        first_names_irish: Vec<String>,
        // first_names_english : Vec<_>,
        // first_names_french : Vec<_>,
        // first_names_scottish : Vec<_>,
        // first_names_welsh : Vec<_>,
        // first_names_sa : Vec<_>,
        // first_names_nz : Vec<_>,
        // first_names_aus : Vec<_>,
        second_names_irish: Vec<String>,
        // second_names_english : Vec<_>,
        // second_names_french : Vec<_>,
        // second_names_scottish : Vec<_>,
        // second_names_welsh : Vec<_>,
        // second_names_sa : Vec<_>,
        // second_names_nz : Vec<_>,
        // second_names_aus : Vec<_>,
    }

    impl NameGenerator {
        // New random name generator
        pub fn new() -> NameGenerator {
            NameGenerator {
                // Load in the file, split by white spaces and convert to strings
                first_names_irish: include_str!("./names/irish_first.in")
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
                second_names_irish: include_str!("./names/irish_last.in")
                    .split_whitespace()
                    .map(String::from)
                    .collect(),
            }
        }

        pub fn get_name(self) -> String {
            let mut rng = rand::thread_rng();

            // Name lengths can be different...
            let indx1 = Uniform::from(0..self.first_names_irish.len());
            let indx2 = Uniform::from(0..self.second_names_irish.len());

            // Grab a random first and last name and concat the two
            let rnd_first_name = self.first_names_irish[indx1.sample(&mut rng)].to_string();
            let rnd_second_name = self.second_names_irish[indx2.sample(&mut rng)].to_string();
            rnd_first_name + " " + &rnd_second_name
        }
    }
}
