// use rand::Rng;
mod com;
mod events;
mod player;
mod random_engine;
mod team;

use random_engine::rng_eng::{
    // roll,
    // RollType,
    AttributeTypes,
    Attributes,
};

use player::player::{
    get_position,
    // FORWARDS,
    Player,
    Position,
};

use team::team::Team;

use events::events::{tackle, scrum, line_out};

fn main() {
    let mut _new_player = Player {
        age: 24,
        name: "Joey Gilroy".to_string(),
        position: [
            Position::BlindSideFlanker,
            Position::OpenSideFlanker,
            Position::Number8,
        ]
        .to_vec(),
        weight: 180,
        attributes: Attributes::new(),
        has_advantage: [AttributeTypes::Strength].to_vec(),
        has_disadvantage: [].to_vec(),
        is_selected: true,
        selected_position: Position::Sub,
    };

    let mut new_team = Team::new();
    new_team.name = "Churchtown Firehawks".to_string();

    println!("Behold the {}:", new_team.name);
    for i in 1..26 {
        let mut temp_player = Player::new();
        // temp_player.name = "Dave Kearney".to_string();
        temp_player.position.push(get_position(i));
        new_team.players.push(temp_player);

        println!(
            "{}, {}: {:?}",
            i,
            new_team.players[usize::from(i - 1)].name,
            new_team.players[usize::from(i - 1)].position[0]
        );
    }

    println!(
        "Team Challange Roll! {}",
        new_team.challange_roll(&AttributeTypes::Strength)
    );
    println!(
        "Forward Challange Roll! {}",
        new_team.forwards_challange_roll(&AttributeTypes::Strength)
    );
    println!(
        "Backs Challange Roll! {}",
        new_team.backs_challange_roll(&AttributeTypes::Strength)
    );

    let (res, _ty) = tackle(&new_team.players[0], &new_team.players[2]);
    println!(
        "{} is tackled by {}...\n{}",
        new_team.players[0].name, new_team.players[2].name, res
    );


    let (res_scrum, res_crit) = scrum(&new_team, &new_team);
    println!("{} -> {:?}", res_scrum, res_crit);


    let (res_scrum, res_crit) = line_out(&new_team, &new_team);
    println!("{} -> {:?}", res_scrum, res_crit);

}
