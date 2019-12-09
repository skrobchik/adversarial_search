pub struct Evaluation{
    depth: i32,
    value: f32,
}

pub enum Player{
    MIN,
    MAX,
}

pub fn minimax<N, FN, IN, FT, FE>(node: &N, depth: i32, successors: &FN, terminal: &FT, evaluation: &FE, player: Player) -> f32
where
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FT: Fn(&N) -> Option<f32>,
    FE: Fn(&N) -> i32,
{
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
            let child_val = minimax(&child, depth-1, successors, terminal, evaluation, match player{
                Player::MIN => Player::MAX,
                Player::MAX => Player::MIN,
            });
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