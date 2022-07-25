pub mod game;
pub mod neat;

use game::Game;
use neat::Neat;
use rand::Rng;

pub struct Solver {
	search_depth: u32
}

impl Solver {
	pub fn new(depth: u32) -> Solver {
		Solver {
			search_depth: depth
		}
	}

	pub fn negamax(self: &Solver, game: &Game) -> usize {
		let moves = game.get_moves();
		
		let mut scores = Vec::new();
		for node in moves {
			scores.push((node.0, -self.negamax_rec(&node.1, f32::NEG_INFINITY, f32::INFINITY, self.search_depth - 1)));
		}
		println!("{:?}", scores);

		let mut best_moves = vec![scores[0]];
		for i in 1..scores.len() {
			if scores[i].1 > best_moves[0].1 {
				best_moves = vec![scores[i]];
			}
			else if scores[i].1 == best_moves[0].1 {
				best_moves.push(scores[i]);
			}
		}

		best_moves[rand::thread_rng().gen_range(0..best_moves.len())].0
	}

	fn negamax_rec(self: &Solver, game: &Game, mut alpha: f32, beta: f32, mut depth: u32) -> f32 {
		if game.check_for_win() {
			return f32::NEG_INFINITY;
		}
		if game.check_for_tie() {
			return 0.0;
		}
		if depth == 0 {
			return game.get_heuristic();
		}
		if alpha >= beta {
			return beta;
		}
		depth -= 1;

		let nodes = game.get_nodes();

		for node in nodes {
			let value = -self.negamax_rec(&node, -beta, -alpha, depth);
			if value >= beta {
				return value;
			}
			if value > alpha {
				alpha = value;
			}
		}

		alpha
	}
}
