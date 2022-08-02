mod solver;

use nannou::prelude::*;
use nannou::state::mouse::Mouse;
use solver::game::Game;
use solver::Solver;
use solver::neat;



fn main() {
	let mut neat_test = neat::Neat::new(2, 1);
	let mut test_network_1 = neat::Network::new(&mut neat_test);
	let mut test_network_0 = neat::Network {
		node_genes: vec![
			neat::Node {
				innovation: 0,
				node_type: neat::Type::Sensor,
				activation_value: None
			},
			neat::Node {
				innovation: 1,
				node_type: neat::Type::Sensor,
				activation_value: None
			},
			neat::Node {
				innovation: 2,
				node_type: neat::Type::Bias,
				activation_value: None
			},
			neat::Node {
				innovation: 3,
				node_type: neat::Type::Output,
				activation_value: None
			},
			neat::Node {
				innovation: 4,
				node_type: neat::Type::Hidden,
				activation_value: None
			},
			neat::Node {
				innovation: 5,
				node_type: neat::Type::Hidden,
				activation_value: None
			}
		],
		connection_genes: vec![
			neat::Connection {
				innovation: 0,
				input: 0,
				output: 4,
				weight: 1.0,
				enabled: true
			},
			neat::Connection {
				innovation: 1,
				input: 0,
				output: 5,
				weight: -1.0,
				enabled: true
			},
			neat::Connection {
				innovation: 2,
				input: 1,
				output: 4,
				weight: -1.0,
				enabled: true
			},
			neat::Connection {
				innovation: 3,
				input: 1,
				output: 5,
				weight: 1.0,
				enabled: true
			},
			neat::Connection {
				innovation: 4,
				input: 4,
				output: 3,
				weight: 10.0,
				enabled: true
			},
			neat::Connection {
				innovation: 5,
				input: 5,
				output: 3,
				weight: 10.0,
				enabled: true
			},
			neat::Connection {
				innovation: 6,
				input: 2,
				output: 3,
				weight: -5.0,
				enabled: true
			}
		],
		num_sensors: 2,
		num_outputs: 1
	};
	let input_vecs = vec![
		vec![0.0, 0.0],
		vec![0.0, 1.0],
		vec![1.0, 0.0],
		vec![1.0, 1.0],
	];
	for input_vec in input_vecs {
		let results = test_network_1.feed_forward(&input_vec);
		println!("{} ^ {} = {}", input_vec[0], input_vec[1], results[0]);
	}
	// nannou::app(model)
	// 	.view(view)
	// 	.run();
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
