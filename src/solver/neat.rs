use rand::prelude::*;
use rand_distr::StandardNormal;

pub struct Neat {
	node_innovation: usize,
	node_mutations: Vec<(usize, usize)>, // (connection split, innovation number)
	pub connection_list: Vec<Connection>,
	connection_lookup: Vec<Vec<usize>>, // for a given node index gives all connections leading away
	num_sensors: usize,
	num_outputs: usize,
	population: Vec<Network>,
	weight_mutation_probability: f64,
	weight_randomization_probability: f64,
	weight_mutation_amount: f32,
	no_crossover_probablility: f64,
	interspecies_mating_rate: f64,
	node_mutation_probability: f64,
	connection_mutation_probability: f64,
	compatability_constant_1: f64,
	compatability_constant_2: f64,
	compatability_constant_3: f64,
	minimum_speciation_distance: f64,
	stagnant_generation_limit: i32,
	current_best_fitness: f32,
	generations_since_last_improvement: i32,
	species_representatives: Vec<Network>,
	network_fitnesses: Vec<f32>
}

impl Neat {
	pub fn new(num_sensors: usize, num_outputs: usize) -> Neat {
		Neat {
			node_innovation: num_sensors + num_outputs + 1,
			node_mutations: Vec::new(),
			connection_list: Vec::new(),
			connection_lookup: vec![Vec::new(); num_sensors + num_outputs + 1],
			num_sensors: num_sensors,
			num_outputs: num_outputs,
			population: Vec::new(),
			weight_mutation_probability: 0.8,
			weight_randomization_probability: 0.1,
			weight_mutation_amount: 0.2,
			no_crossover_probablility: 0.25,
			interspecies_mating_rate: 0.001,
			node_mutation_probability: 0.03,
			connection_mutation_probability: 0.05,
			compatability_constant_1: 1.0,
			compatability_constant_2: 1.0,
			compatability_constant_3: 0.4,
			minimum_speciation_distance: 3.0,
			stagnant_generation_limit: 15,
			current_best_fitness: 0.0,
			generations_since_last_improvement: 0,
			species_representatives: Vec::new(),
			network_fitnesses: Vec::new()
		}
	}

	pub fn generate_population(&mut self, num_members: usize) {
		self.population.reserve(num_members);
		for _ in 0..num_members {
			let new_network = Network::new(self);
			self.population.push(new_network);
		}
	}

	pub fn get_network(&mut self, network: usize) -> &mut Network {
		&mut self.population[network]
	}

	pub fn find_connection(&self, input_node: usize, output_node: usize) -> Option<usize> {
		for connection in &self.connection_lookup[input_node] {
			if self.connection_list[*connection].output == output_node {
				return Some(*connection);
			}
		}
		None
	}

	pub fn calculate_fitnesses(&mut self, fitness_function: fn(&mut Network) -> f32) {

	}

	pub fn _get_xor_network() -> Network {
		Network {
			node_genes: vec![
				Node {
					innovation: 0,
					node_type: Type::Sensor,
					activation_value: None
				},
				Node {
					innovation: 1,
					node_type: Type::Sensor,
					activation_value: None
				},
				Node {
					innovation: 2,
					node_type: Type::Bias,
					activation_value: Some(1.0)
				},
				Node {
					innovation: 3,
					node_type: Type::Output,
					activation_value: None
				},
				Node {
					innovation: 4,
					node_type: Type::Hidden,
					activation_value: None
				}
			],
			connection_genes: vec![
				Connection {
					innovation: 0,
					input: 0,
					output: 3,
					weight: 1.0,
					enabled: true
				},
				Connection {
					innovation: 1,
					input: 1,
					output: 3,
					weight: 1.0,
					enabled: true
				},
				Connection {
					innovation: 2,
					input: 0,
					output: 4,
					weight: 1.0,
					enabled: true
				},
				Connection {
					innovation: 3,
					input: 1,
					output: 4,
					weight: 1.0,
					enabled: true
				},
				Connection {
					innovation: 4,
					input: 2,
					output: 4,
					weight: -1.0,
					enabled: true
				},
				Connection {
					innovation: 5,
					input: 4,
					output: 3,
					weight: -2.0,
					enabled: true
				}
			],
			num_sensors: 2,
			num_outputs: 1
		}
	}
}

#[derive(Copy, Clone)]
pub struct Connection {
	innovation: usize,
	input: usize,
	output: usize,
	pub weight: f32,
	enabled: bool
}

#[derive(Copy, Clone)]
pub enum Type {
	Sensor,
	Output,
	Hidden,
	Bias
}
impl Type {
	pub fn activation(self: &Type, input: f32) -> f32 {
		match *self {
			Type::Sensor => input,
			Type::Output => 1.0 / (1.0 + (-input).exp()),
			Type::Hidden => input.max(0.0),
			Type::Bias => 1.0
		}
	}
}

#[derive(Copy, Clone)]
pub struct Node {
	innovation: usize,
	node_type: Type,
	activation_value: Option<f32>
}
impl Node {
	pub fn give_input(self: &mut Node, input: f32) {
		self.activation_value = Some(self.node_type.activation(input));
	}

	pub fn clear(&mut self) {
		self.activation_value = None;
	}
}

pub struct Network {
	node_genes: Vec<Node>,
	pub connection_genes: Vec<Connection>,
	num_sensors: usize,
	num_outputs: usize
}

impl Network {
	pub fn new(global: &mut Neat) -> Network {
		let mut nodes = Vec::with_capacity(global.num_sensors + global.num_outputs + 1);
		for i in 0..global.num_sensors + global.num_outputs + 1 {
			nodes.push(Node {
				innovation: i,
				node_type:
					if (0..global.num_sensors).contains(&i) {
						Type::Sensor
					}
					else if global.num_sensors == i {
						Type::Bias
					}
					else {
						Type::Output
					},
				activation_value: None
			});
		}

		let mut network = Network {
			node_genes: nodes,
			connection_genes: Vec::new(),
			num_sensors: global.num_sensors,
			num_outputs: global.num_outputs
		};
		network.add_connection(global);

		network
	}

	pub fn feed_forward(self: &mut Network, inputs: &Vec<f32>) -> Vec<f32> {
		for i in 0..self.num_sensors {
			self.node_genes[i].give_input(inputs[i]);
		}
		self.node_genes[self.num_sensors].give_input(1.0);

		let mut output = Vec::with_capacity(self.num_outputs);
		for output_node in self.num_sensors + 1..self.num_outputs + self.num_sensors + 1 {
			output.push(self.evaluate_node(output_node));
		}
		for node in &mut self.node_genes {
			node.clear();
		}
		
		output
	}

	fn evaluate_node(self: &mut Network, node: usize) -> f32 {
		let mut acc = 0.0;
		for i in 0..self.connection_genes.len() {
			if self.connection_genes[i].enabled && self.connection_genes[i].output == node {
				let prev_node = self.connection_genes[i].input;
				acc += self.connection_genes[i].weight *
					self.node_genes[prev_node].activation_value
					.unwrap_or_else(|| self.evaluate_node(prev_node));
			}
		}

		self.node_genes[node].give_input(acc);

		self.node_genes[node].activation_value.unwrap()
	}

	pub fn add_connection(self: &mut Network, global: &mut Neat) {
		let to_node = rand::thread_rng().gen_range(self.num_sensors + 1, self.node_genes.len());

		let mut possible_from_nodes = vec![true; global.node_innovation];

		for i in self.num_sensors + 1..self.num_outputs + self.num_sensors + 1 { // connection can't come from output node
			possible_from_nodes[i] = false;
		}

		let mut nodes_to_search = vec![to_node];
		while let Some(current_node) = nodes_to_search.pop() {
			if possible_from_nodes[current_node] { // node hasn't been searched
				possible_from_nodes[current_node] = false;

				for connection_ptr in &global.connection_lookup[current_node] {
					nodes_to_search.push(global.connection_list[*connection_ptr].output);
				}
			}
		}

		let mut from_nodes = Vec::new();
		for i in 0..possible_from_nodes.len() {
			if possible_from_nodes[i] {
				from_nodes.push(i);
			}
		}

		let from_node = from_nodes[rand::thread_rng().gen_range(0, from_nodes.len())];

		if let Some(innovation_num) = global.find_connection(from_node, to_node) {
			for connection in &mut self.connection_genes {
				if connection.innovation == innovation_num {
					connection.weight = thread_rng().sample(StandardNormal);
					connection.enabled = true;
					return;
				}
			}
			self.connection_genes.push(Connection {
				innovation: innovation_num,
				input: from_node,
				output: to_node,
				weight: thread_rng().sample(StandardNormal),
				enabled: true
			});
			return;
		}
		self.connection_genes.push(Connection {
			innovation: global.connection_list.len(),
			input: from_node,
			output: to_node,
			weight: thread_rng().sample(StandardNormal),
			enabled: true
		});
		global.connection_lookup[from_node].push(global.connection_list.len());
		global.connection_list.push(Connection {
			innovation: global.connection_list.len(),
			input: from_node,
			output: to_node,
			weight: 0.0,
			enabled: true
		});
	}

	pub fn add_node(&mut self, global: &mut Neat) {
		let mut enabled_connections = Vec::new();
		for i in 0..self.connection_genes.len() {
			if self.connection_genes[i].enabled {
				enabled_connections.push(i);
			}
		}
		let connection_to_split = enabled_connections[rand::thread_rng().gen_range(0, enabled_connections.len())];
		self.connection_genes[connection_to_split].enabled = false;
		let input_node = self.connection_genes[connection_to_split].input;
		let output_node = self.connection_genes[connection_to_split].output;
		let connection_weight = self.connection_genes[connection_to_split].weight;

		let previous_mutation = global.node_mutations.iter().find(|&&i| i.0 == connection_to_split);
		let connection_innovation = match previous_mutation {
			Some(x) => global.find_connection(input_node, x.1),
			None => None
		}.unwrap_or(global.connection_list.len());
		let node_innovation = match previous_mutation {
			Some(x) => x.1,
			None => global.node_innovation
		};

		let new_node = Node {
			innovation: node_innovation,
			node_type: Type::Hidden,
			activation_value: None
		};
		let connection_to = Connection {
			innovation: connection_innovation,
			input: input_node,
			output: node_innovation,
			weight: 1.0,
			enabled: true
		};
		let connection_from = Connection {
			innovation: connection_innovation + 1,
			input: node_innovation,
			output: output_node,
			weight: connection_weight,
			enabled: true
		};

		if None == previous_mutation {
			global.node_innovation += 1;
			global.connection_list.push(connection_to);
			global.connection_list.push(connection_from);
			global.connection_lookup[input_node].push(connection_innovation);
			global.connection_lookup.push(vec![connection_innovation + 1]);
			global.node_mutations.push((connection_to_split, node_innovation));
		}

		self.node_genes.push(new_node);
		self.connection_genes.push(connection_to);
		self.connection_genes.push(connection_from);
	}

	pub fn cross(&mut self, other: &mut Self, global: &mut Neat) -> Self {
		self.connection_genes.sort_by_key(|x| x.innovation);
		other.connection_genes.sort_by_key(|x| x.innovation);
		let mut crossed_connections = self.connection_genes.clone();
		let mut i = 0;
		for connection in &mut crossed_connections {
			match loop {
				i += 1;
				if i >= other.connection_genes.len() {
					break None;
				}
				if connection.innovation == other.connection_genes[i].innovation {
					break Some(i);
				}
			} {
				Some(x) => if rand::thread_rng().gen_bool(0.5) {
					connection.weight = other.connection_genes[x].weight;
				},
				None => break
			};
		}

		let mut new_network = Network {
			node_genes: self.node_genes.clone(),
			connection_genes: crossed_connections,
			num_sensors: self.num_sensors,
			num_outputs: self.num_outputs
		};

		new_network.mutate_network(global);

		new_network
	}

	pub fn mutate_network(&mut self, global: &mut Neat) {
		if rand::thread_rng().gen_bool(global.weight_mutation_probability) {
			let connection_to_mutate = rand::thread_rng().gen_range(0, self.connection_genes.len());
			if rand::thread_rng().gen_bool(global.weight_randomization_probability) {
				self.connection_genes[connection_to_mutate].weight = thread_rng().sample(StandardNormal);
			}
			else {
				self.connection_genes[connection_to_mutate].weight += thread_rng().sample::<f32, _>(StandardNormal) * global.weight_mutation_amount;
			}
		}

		if rand::thread_rng().gen_bool(global.node_mutation_probability) {
			self.add_node(global);
		}
		if rand::thread_rng().gen_bool(global.connection_mutation_probability) {
			self.add_connection(global);
		}
	}
}
