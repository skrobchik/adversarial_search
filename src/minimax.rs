pub struct Evaluation{
    depth: i32,
    value: f32,
}

enum Player{
    MIN,
    MAX,
}

fn is_maximizing_player(player: &Player) -> bool{
    match player{
        Player::MAX => true,
        Player::MIN => false,
    }
}

pub fn minimax<N, FN, IN, FT, FE>(node: &N, depth: i32, successors: &FN, terminal: &FT, evaluation: &FE, maximizing_player: bool) -> f32
where
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FT: Fn(&N) -> Option<f32>,
    FE: Fn(&N) -> i32,
{
    let player = match maximizing_player{
        true => Player::MAX,
        false => Player::MIN,
    };
    let terminality = terminal(&node);
    if terminality.is_some() {
        return terminality.unwrap();
    }
    else{
        let mut val = match player{
            Player::MIN => std::f32::INFINITY,
            Player::MAX => -std::f32::INFINITY,
        };
        for child in successors(&node){
            let child_val = minimax(&child, depth-1, successors, terminal, evaluation, is_maximizing_player(&player));
            let is_better_value: bool = match player{
                Player::MIN => child_val < val,
                Player::MAX => child_val > val,
            };
            if is_better_value{
                val = child_val;
            }
        }
        return val;
    }
}