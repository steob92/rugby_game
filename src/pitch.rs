// Module to handle anything related to the pitch

pub mod pitch {

    // Define pitch coordinates
    const PITCH_WIDTH: f32 = 70.;
    const PITCH_LENGTH: f32 = 100.;
    const HOME_22: f32 = 22.;
    const AWAY_22: f32 = 78.;
    const HOME_5M: f32 = 5.;
    const AWAY_5M: f32 = 95.;
    const HALF_WAY: f32 = 50.;

    pub struct PitchPosition {
        x: f32,
        y: f32,
    }

    impl PitchPosition {
        pub fn new() -> PitchPosition {
            PitchPosition { x: 0., y: 0. }
        }

        // Find the angle to the goal
        pub fn goal_angle(&self, is_home: &bool) -> f32 {
            if self.x == 0. {
                return 90.;
            }

            let ydist = 0.5 * PITCH_WIDTH - self.y;
            let ang = if *is_home {
                ydist / (PITCH_LENGTH - self.x)
            } else {
                ydist / (-self.x)
            };
            // Get the absolute degrees
            ang.atan().to_degrees().abs()
        }

        // Find the distance to the goal
        pub fn goal_dist(&self, is_home: &bool) -> f32 {
            // Get the absolute distance
            match *is_home {
                true => ((self.x - PITCH_LENGTH).powf(2.) + (self.y - 0.5 * PITCH_WIDTH).powf(2.))
                    .sqrt()
                    .abs(),
                false => ((self.x).powf(2.) + (self.y - 0.5 * PITCH_WIDTH).powf(2.))
                    .sqrt()
                    .abs(),
            }
        }

        // Get the difficulty of a kick
        pub fn goal_kick_difficutly(&self, is_home: &bool) -> i32 {
            let ang = self.goal_angle(is_home);
            let dist = self.goal_dist(is_home);

            // First pass on the angle
            let mut diff = match ang as i32 {
                0..=30 => 10,
                31..=45 => 12,
                46..=60 => 14,
                61..=75 => 16,
                76..=80 => 18,
                // Anything beyond 80 degrees should be almost impossible
                81..=85 => 20,
                _ => 30,
            };

            // Add in the distance modifier
            diff += match dist as i32 {
                // 0-22
                0..=22 => 0,
                // 22 - 30
                23..=30 => 2,
                // 30 - 40
                31..=40 => 4,
                // 40 - 50
                41..=50 => 6,
                // Outside of the half...
                _ => 10,
            };
            // Return the difficulty
            diff
        }
    }
}
