// Module to handle the match events

pub mod events {
    use crate::player::player::{Player, Position};
    use crate::random_engine::rng_eng::{AttributeTypes, RollResult};
    use crate::team::team::Team;


    // Critical values for a scrum event
    const SCRUM_CRIT : i32 = 10;
    const SCRUM_PUT_IN_ADV : i32 = 10;


    // Critical values for a line out event
    const LINE_OUT_TROW_CRIT : i32 = 10;

    // Critical values for a maul event
    const MAUL_CRIT :i32 = 10;

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
    // This is a contested forwards challange
    // The attacking team should have some form of advantange since they have the put in
    // ToDo: Implement a PitchPosition that will proxy for presure
    pub fn scrum( att_team :&Team, def_team :&Team) -> (bool, RollResult) {

        let att_chall = att_team.forwards_challange_roll(&AttributeTypes::Strength) + SCRUM_PUT_IN_ADV;
        let def_chall = def_team.forwards_challange_roll(&AttributeTypes::Strength);

        // Check if the difference is greater than a critcal value
        // Return a Critical Success otherwise
        let res = att_chall > def_chall;


        let crit = match  (att_chall - def_chall).abs() > SCRUM_CRIT {
            true => RollResult::CriticalSuccess,
            false => RollResult::Flat,
        };

        (res, crit)
    }


    // Line Out
    // Contested Challange Roll
    // Challange roll for the throw, the catch and play after...
    pub fn line_out( att_team :&Team, def_team :&Team) -> (bool, RollResult){

        // Throw in first check if the throw in is successful
        // Fine the hooker on the att_team
        let hooker = att_team.get_player(Position::Hooker);
        let throw = hooker.challange_roll(AttributeTypes::Dexterity);

        match throw.1 {
            // Perfect throw
            RollResult::CriticalSuccess => {
                (true, RollResult::CriticalSuccess)
            },
            // Terrible throw
            RollResult::CriticalFail => {
                (false, RollResult::CriticalFail)
            },
            // Contestable throw
            RollResult::Flat => {
                (contest_line_out(throw.0, att_team, def_team), RollResult::Flat)
            },
        }
    }

    // Contested line out
    // Dex challange roll to see which team will recover the line out
    // Adding offsets to give advantage to the throwing teams
    fn contest_line_out(put_in :i32, att_team :&Team, def_team :&Team) -> bool {
        // Contested line out throw
        let att_chal = att_team.forwards_challange_roll(&AttributeTypes::Dexterity);
        let def_chal = def_team.forwards_challange_roll(&AttributeTypes::Dexterity);

        // Add the throw score and a balancing score
        att_chal + put_in + LINE_OUT_TROW_CRIT > def_chal
        
    }


    // Generic group check
    fn group_check(group: Vec<&Player>, attr: &AttributeTypes) -> i32{
        group.iter().map(|x| x.challange_roll(*attr).0).sum()
    }

    // Generic group contest
    // Is this function needed?
    // Requires two references whereas group check will just require one...
    // Attacking and defending terms still relvent?
    // Control absolute results with critical values on return
    // fn  group_contest(att_team :Vec<&Player>, def_team :Vec<&Player>, att_attr :&AttributeTypes, def_attr :&AttributeTypes) -> i32 {
    //     let att_check = group_check(att_team, att_attr);
    //     let def_check = group_check(def_team, def_attr);
    //     att_check - def_check
    // }

    // Maul
    // Contested strength test between two groups of players
    pub fn maul(att_group: Vec<&Player>, def_group: Vec<&Player>) -> (bool, RollResult) {
        let res = group_check(att_group, &AttributeTypes::Strength) -  group_check(def_group, &AttributeTypes::Strength);

        // Did the maul event succeed?
        let suc = if res < 0 {
            false
        } else {
            true
        };

        let roll  = if res.abs() < MAUL_CRIT{
            RollResult::Flat
        } else if res > 0 {
            RollResult::CriticalSuccess
        } else{
            RollResult::CriticalFail
        };

        (suc, roll)
    }

}
