use crate::utils::min;
use crate::utils::max;

enum Player {
    MIN,
    MAX,
}

fn is_maximizing_player(player: &Player) -> bool {
    match player {
        Player::MAX => true,
        Player::MIN => false,
    }
}

pub fn alphabeta<N, FN, IN, FT, FE>(
    node: &N,
    depth: i32,
    successors: &FN,
    terminal: &FT,
    evaluation: &FE,
    maximizing_player: bool,
    mut alpha: f32,
    mut beta: f32,
) -> f32
where
    FN: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FT: Fn(&N) -> Option<f32>,
    FE: Fn(&N) -> i32,
{
    let player = match maximizing_player {
        true => Player::MAX,
        false => Player::MIN,
    };
    let terminality = terminal(&node);
    if terminality.is_some() {
        return terminality.unwrap();
    }
    let mut val = match player {
        Player::MIN => std::f32::INFINITY,
        Player::MAX => -std::f32::INFINITY,
    };
    for child in successors(&node) {
        let child_val = alphabeta(
            &child,
            depth - 1,
            successors,
            terminal,
            evaluation,
            is_maximizing_player(&player),
            alpha,
            beta,
        );
        match player {
            Player::MAX => {
                val = max(val, child_val);
                alpha = max(alpha, val);
            }
            Player::MIN => {
                val = min(val, child_val);
                beta = min(beta, val);
            }
        }
        if alpha >= beta {
            break;
        }
    }
    val
}
