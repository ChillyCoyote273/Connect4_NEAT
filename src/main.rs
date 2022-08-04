mod solver;

use nannou::prelude::*;
use nannou::state::mouse::Mouse;
use solver::game::Game;
use solver::Solver;
use solver::neat;



fn main() {
	let mut neat_test = neat::Neat::new(2, 1);
	neat_test.generate_population(10);
	neat_test.calculate_fitnesses(calculate_fitness);
	// nannou::app(model)
	// 	.view(view)
	// 	.run();
}

fn reverse_mean_square_error(target: &Vec<f32>, output: Vec<f32>) -> f32 {
	let mut acc = 0.0;
	for i in 0..target.len() {
		acc += 1.0 - (target[i] - output[i]) * (target[i] - output[i]);
	}
	acc
}
fn calculate_fitness(network: &mut neat::Network) -> f32 {
	let data_vec:Vec<(Vec<f32>, Vec<f32>)> = vec![
		(vec![0.0, 0.0], vec![0.0]),
		(vec![0.0, 1.0], vec![1.0]),
		(vec![1.0, 0.0], vec![1.0]),
		(vec![1.0, 1.0], vec![0.0])
	];

	let mut total_fitness = 0.0;
	for data in data_vec {
		total_fitness += reverse_mean_square_error(&data.1, network.feed_forward(&data.0));
	}

	total_fitness
}



struct Model {
	_window_id: window::Id,
	connect: Game,
	solver: Solver,
	game_over: bool,
	ai_turn: i32
}

fn model(app: &App) -> Model {
	let id = app.new_window()
		.size(900, 775)
		.event(window_event)
		.build()
		.unwrap();
	
    let mut model = Model {
		_window_id: id,
		connect: Game::new(),
		solver: Solver::new(11),
		game_over: false,
		ai_turn: 2
	};

	if model.ai_turn == 1 {
		let best_move = model.solver.negamax(&model.connect);
		model.connect.play_piece(best_move);
	}

	model
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
	match event {
		MousePressed(button) => {
			if button == MouseButton::Left && !model.game_over && model.connect.get_turn() != model.ai_turn {
				let column = clamp(((app.mouse.x + app.window_rect().w() * 0.5 - 12.5) / 125.0) as usize, 0, 6);
				model.game_over = model.connect.play_piece(column);
				
				if !model.game_over && model.connect.get_turn() == model.ai_turn {
					let best_move = model.solver.negamax(&model.connect);
					model.game_over = model.connect.play_piece(best_move);
				}
			}
		}
		_ => {}
	}
}

fn view(app: &App, model: &Model, frame: Frame) {
	let draw = app.draw();
	let win = app.window_rect();

	draw_board(&app.mouse, &draw, &win, &model);
	
	draw.to_frame(app, &frame).unwrap();
}



fn draw_board(mouse: &Mouse, draw: &Draw, win: &Rect, model: &Model) {
	draw.rect()
		.xy(win.xy())
		.wh(win.wh())
		.color(ROYALBLUE);
	
	let board = model.connect.get_board();

	let circle = Rect::from_w_h(100.0f32, 100.0f32).top_left_of(*win).shift_x(25.0).shift_y(-25.0);
	for i in 0..7 {
		let pos_x = 125.0 * i as f32;
		for j in 0..6 {
			let pos_y = -125.0 * (5 - j) as f32;
			draw.ellipse()
				.xy(circle.shift_x(pos_x).shift_y(pos_y).xy())
				.wh(circle.wh())
				.color(
					match board[i][j] {
						1 => RED,
						2 => YELLOW,
						_ => BLACK
					}
				);
		}
	}

	if !model.game_over && model.connect.get_turn() != model.ai_turn {
		let column = clamp(((mouse.x + win.w() / 2.0 - 12.5) / 125.0) as usize, 0, 6);
		let mut empty_slot = -1;
		for i in 0..6 {
			if board[column][i] == 0 {
				empty_slot = i as i32;
				break;
			}
		}

		if empty_slot != -1 {
			draw.ellipse()
				.xy(circle.shift_x(125.0 * column as f32).shift_y(-125.0 * (5 - empty_slot) as f32).xy())
				.w_h(50.0, 50.0)
				.color(
					match model.connect.get_turn() {
						1 => RED,
						2 => YELLOW,
						_ => WHITE
					}
				);
		}
	}
}
