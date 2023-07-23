pub mod player {
    use std::fmt; // Import `fmt`

    use crate::{
        random_engine::rnd_name::NameGenerator,
        random_engine::rng_eng::{AttributeTypes, Attributes, RollResult, RollType}
    };

    #[derive(Clone, PartialEq)]
    #[allow(dead_code)]
    // Enum of the available positions

    pub enum Position {
        LooseHead,
        TightHead,
        Hooker,
        SecondRow1,
        SecondRow2,
        BlindSideFlanker,
        OpenSideFlanker,
        Number8,
        ScrumHalf,
        FlyHalf,
        InsideCentre,
        OutsideCentre,
        LeftWinger,
        RightWinger,
        FullBack,
        Sub, // sub will be the default and fallback for non-selected players
    }

    pub const FORWARDS: [Position; 8] = [
        Position::LooseHead,
        Position::TightHead,
        Position::Hooker,
        Position::SecondRow1,
        Position::SecondRow2,
        Position::BlindSideFlanker,
        Position::OpenSideFlanker,
        Position::Number8,
    ];

    pub fn get_position(num: u8) -> Position {
        match num {
            1 => Position::LooseHead,
            3 => Position::TightHead,
            2 => Position::Hooker,
            4 => Position::SecondRow1,
            5 => Position::SecondRow2,
            6 => Position::BlindSideFlanker,
            7 => Position::OpenSideFlanker,
            8 => Position::Number8,
            9 => Position::ScrumHalf,
            10 => Position::FlyHalf,
            12 => Position::InsideCentre,
            13 => Position::OutsideCentre,
            11 => Position::LeftWinger,
            14 => Position::RightWinger,
            15 => Position::FullBack,
            _ => Position::Sub,
        }
    }

    pub fn get_number(pos: Position) -> u8 {
        match pos {
            Position::LooseHead => 1,
            Position::TightHead => 3,
            Position::Hooker => 2,
            Position::SecondRow1 => 4,
            Position::SecondRow2 => 5,
            Position::BlindSideFlanker => 6,
            Position::OpenSideFlanker => 7,
            Position::Number8 => 8,
            Position::ScrumHalf => 9,
            Position::FlyHalf => 10,
            Position::InsideCentre => 12,
            Position::OutsideCentre => 13,
            Position::LeftWinger => 11,
            Position::RightWinger => 14,
            Position::FullBack => 15,
            Position::Sub => 16,
        }
    }

    // Implement `Display` for `Position`.
    // Retrun the postion number for a given position
    impl fmt::Debug for Position {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Position::LooseHead => write!(f, "1"),
                Position::TightHead => write!(f, "3"),
                Position::Hooker => write!(f, "2"),
                Position::SecondRow1 => write!(f, "4"),
                Position::SecondRow2 => write!(f, "5"),
                Position::OpenSideFlanker => write!(f, "6"),
                Position::BlindSideFlanker => write!(f, "7"),
                Position::Number8 => write!(f, "8"),
                Position::ScrumHalf => write!(f, "9"),
                Position::FlyHalf => write!(f, "10"),
                Position::InsideCentre => write!(f, "12"),
                Position::OutsideCentre => write!(f, "13"),
                Position::LeftWinger => write!(f, "11"),
                Position::RightWinger => write!(f, "14"),
                Position::FullBack => write!(f, "15"),
                Position::Sub => write!(f, "16+"),
            }
        }
    }

    // Struct for a player
    #[derive(Clone)]
    pub struct Player {
        pub age: u8,
        pub name: String,
        pub position: Vec<Position>,
        pub weight: u32,
        pub attributes: Attributes,
        pub has_advantage: Vec<AttributeTypes>,
        pub has_disadvantage: Vec<AttributeTypes>,
        pub is_selected: bool,
        pub selected_position: Position,
    }

    impl Player {
        // Default empty player
        pub fn new() -> Player {
            Player {
                age: 0,
                // name : "".to_string(),
                name: NameGenerator::new().get_name(),
                position: [].to_vec(),
                weight: 0,
                attributes: Attributes::new(),
                has_advantage: [].to_vec(),
                has_disadvantage: [].to_vec(),
                is_selected: true,
                selected_position: Position::Sub,
            }
        }

        // Challange roll for this player
        pub fn challange_roll(&self, attr: &AttributeTypes) -> (i32, RollResult) {
            // Should this be rewritten as a match?
            // Check if the player has advantage or disadvantage on the roll
            // Player will have advantage only if they can have advantage on the attribute and they can play the position
            let roll_type = if self.has_advantage.contains(&attr)
                && self.position.contains(&self.selected_position)
            {
                RollType::Advantage
            } else if self.has_disadvantage.contains(attr)
                || !self.position.contains(&self.selected_position)
            {
                RollType::Disavantage
            } else {
                RollType::Flat
            };

            self.attributes.challange_roll(*attr, roll_type)
        }
    }
}
