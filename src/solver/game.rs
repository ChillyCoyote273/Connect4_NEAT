pub struct Game {
	board: u64,
	mask: u64,
	first_player: bool
}

impl Game {
	pub fn new() -> Game {
		Game {
			board: 0,
			mask: 0,
			first_player: true
		}
	}

	pub fn copy(self: &Game) -> Game {
		Game {
			board: self.board,
			mask: self.mask,
			first_player: self.first_player
		}
	}

	pub fn get_turn(self: &Game) -> i32 {
		if self.first_player {1} else {2}
	}

	pub fn get_board(self: &Game) -> [[u8; 6]; 7] {
		let mut board = [[0; 6]; 7];
		for i in 0..7 {
			for j in 0..6 {
				let position = 1 << (j + i * 7);
				if self.mask & position != 0 {
					let bit = if self.board & position != 0 {1} else {2};
					board[i][j] = if self.first_player {bit} else {3 - bit};
				}
			}
		}
		board
	}

	pub fn play_piece(self: &mut Game, column: usize) -> bool {
		let top_mask = 1 << (column * 7 + 5);
		if self.mask & top_mask == 0 {
			self.board ^= self.mask;
			self.mask |= self.mask + (1 << column * 7);
			self.first_player = !self.first_player;
			return self.check_for_win()
		}
		false
	}

	pub fn check_for_win(self: &Game) -> bool {
		let board = self.board ^ self.mask;

		let mut n = board & (board >> 7);
		if n & (n >> 14) != 0 { return true; }

		n = board & (board >> 6);
		if n & (n >> 12) != 0 { return true; }

		n = board & (board >> 8);
		if n & (n >> 16) != 0 { return true; }

		n = board & (board >> 1);
		if n & (n >> 2) != 0 { return true; }

		false
	}

	pub fn check_for_tie(self: &Game) -> bool {
		if self.mask == 0b0111111011111101111110111111011111101111110111111 {
			return true;
		}

		false
	}

	fn play_piece_no_check(self: &mut Game, column: usize) -> bool {
		let top_mask = 1 << (column * 7 + 5);
		if self.mask & top_mask == 0 {
			self.board ^= self.mask;
			self.mask |= self.mask + (1 << column * 7);
			self.first_player = !self.first_player;
			return true;
		}
		false
	}

	pub fn get_nodes(self: &Game) -> Vec<Game> {
		let mut nodes = Vec::new();
		for i in 0..7 {
			let mut game = self.copy();
			if game.play_piece_no_check(i) {
				nodes.push(game);
			}
		}
		nodes
	}

	pub fn get_moves(self: &Game) -> Vec<(usize, Game)> {
		let mut moves = Vec::new();
		for i in 0..7 {
			let mut game = self.copy();
			if game.play_piece_no_check(i) {
				moves.push((i, game));
			}
		}
		moves
	}

	pub fn get_heuristic(self: &Game) -> f32 {
		let mut score = Game::get_one_score(self.board, self.mask);

		score -= Game::get_one_score(self.board ^ self.mask, self.mask);
		
		score
	}

	fn get_one_score(board: u64, mut mask: u64) -> f32 {
		mask = !mask & 0b_0111111_0111111_0111111_0111111_0111111_0111111_0111111;
		
		let mut n = board & (board >> 7) & (board >> 14);
		let mut score = Game::pop_count(n & mask >> 21) + Game::pop_count(n & mask << 7);
		
		n = board & (board >> 6) & (board >> 12);
		score += Game::pop_count(n & mask >> 18) + Game::pop_count(n & mask << 6);
		
		n = board & (board >> 8) & (board >> 16);
		score += Game::pop_count(n & mask >> 24) + Game::pop_count(n & mask << 8);
		
		n = board & (board >> 1) & (board >> 2);
		score += Game::pop_count(n & mask >> 3) + Game::pop_count(n & mask << 1);

		score as f32
	}

	fn pop_count(mut x: u64) -> u64 {
		let m1 = 0x5555555555555555;
		let m2 = 0x3333333333333333;
		let m4 = 0x0f0f0f0f0f0f0f0f;
		let h01 = 0x0101010101010101;
		x -= (x >> 1) & m1;
		x = (x & m2) + ((x >> 2) & m2);
		x = (x + (x >> 4)) & m4;
		(x * h01) >> 56
	}
}
