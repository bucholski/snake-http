use super::*;
use rand::{seq::IteratorRandom, thread_rng};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn set_opposite_direction(current_direction: Direction) -> Direction {
    match current_direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    }
}

pub fn select_move(direction_queue: &DirQueue, current_direction: Direction) -> Direction {
    let mut rng = thread_rng();
    let opposite_direction = set_opposite_direction(current_direction);

    let direction_queue = direction_queue.lock().unwrap();
    let choose_one_direction = direction_queue
        .iter()
        .filter(|direction| **direction != opposite_direction)
        .choose(&mut rng);

    match choose_one_direction {
        Some(direction) => *direction,
        None => current_direction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn debug_queue() -> DirQueue {
        Arc::new(Mutex::new(vec![
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ]))
    }
    #[test]
    fn select_move_test() {
        for _ in 0..1000 {
            let current_direction = Direction::Up;
            let selected_move = select_move(&debug_queue(), current_direction);
            assert_ne!(selected_move, Direction::Down);
        }
    }
}
