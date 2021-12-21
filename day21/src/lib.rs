pub fn process(input: &str) -> Option<u64> {
    let (p1_pos, p2_pos) = parse_input(input);
    let mut die_num = 1;
    let mut game = GameState::new(p1_pos, p2_pos);

    loop {
        let move_amt = die_num + die_num + 1 + die_num + 2;
        game = game.next(move_amt, false);

        if game.player1.score >= 1000 || game.player2.score >= 1000 {
            break;
        }

        die_num += 3;
    }

    let loser = if game.player1.score > game.player2.score {
        game.player2.score
    } else {
        game.player1.score
    };

    Some(((die_num + 2) * loser) as u64)
}

pub fn process_pt2(input: &str) -> Option<u64> {
    let (p1_pos, p2_pos) = parse_input(input);
    let mut state_stack: Vec<GameState> = vec![GameState::new(p1_pos, p2_pos)];
    let mut player_wins = (0, 0);

    while state_stack.len() > 0 {
        let mut curr_state = state_stack.pop().unwrap();
        for roll in 3..=9 {
            let next_state = curr_state.next(roll, true);
            if next_state.player1.score >= 21 || next_state.player2.score >= 21 {
                if next_state.player1.score > next_state.player2.score {
                    player_wins.0 += next_state.num_universes;
                } else {
                    player_wins.1 += next_state.num_universes;
                }
            } else {
                state_stack.push(next_state);
            }
        }
    }

    let winner = if player_wins.0 > player_wins.1 {
        player_wins.0
    } else {
        player_wins.1
    };

    Some(winner)
}

fn parse_input(input: &str) -> (u32, u32) {
    let f = input.split_once('\n').unwrap();
    let p1_pos: u32 =
        f.0.chars()
            .last()
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap()
            - 1;
    let p2_pos: u32 =
        f.1.chars()
            .last()
            .unwrap()
            .to_string()
            .parse::<u32>()
            .unwrap()
            - 1;

    (p1_pos, p2_pos)
}

#[derive(Debug, Clone)]
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
    turn: u32,
    num_universes: u64,
}

#[derive(Debug, Clone)]
struct PlayerState {
    location: u32,
    score: u32,
}

impl PlayerState {
    fn new(location: u32) -> Self {
        PlayerState { score: 0, location }
    }

    fn run(&mut self, roll: u32) {
        self.location += roll;
        self.location = if self.location >= 10 {
            self.location % 10
        } else {
            self.location
        };

        self.score = self.score + self.location + 1;
    }
}

impl GameState {
    fn new(p1_start: u32, p2_start: u32) -> Self {
        GameState {
            player1: PlayerState::new(p1_start),
            player2: PlayerState::new(p2_start),
            num_universes: 1,
            turn: 0,
        }
    }

    fn next(&mut self, roll: u32, more_universes: bool) -> GameState {
        let mut next_state = self.clone();

        if next_state.turn % 2 == 0 {
            next_state.player1.run(roll);
        } else {
            next_state.player2.run(roll);
        }

        next_state.turn += 1;

        if more_universes {
            let possible_universes = vec![0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

            next_state.num_universes *= possible_universes[roll as usize];
        }

        next_state
    }
}
