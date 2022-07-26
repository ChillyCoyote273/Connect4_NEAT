use rand::Rng;

pub struct Neat {
	node_list: Vec<Node>,
	connection_list: Vec<Connection>,
	connection_lookup: Vec<Vec<usize>> // for a given node index gives all connections leading away
}

impl Neat {

}

struct Connection {
	innovation: usize,
	input: usize,
	output: usize,
	weight: f32,
	enabled: bool
}

enum Type {
	Sensor,
	Output,
	Hidden
}

impl Type {
	pub fn copy(self: &Type) -> Type {
		match self {
			Type::Sensor => Type::Sensor,
			Type::Output => Type::Output,
			Type::Hidden => Type::Hidden
		}
	}

	pub fn activation(self: &Type, input: f32) -> f32 {
		match *self {
			Type::Sensor => input,
			Type::Output => 1.0 / (1.0 + (-input).exp()),
			Type::Hidden => input.max(0.0)
		}
	}
}

struct Node {
	innovation: usize,
	node_type: Type,
	activation_value: Option<f32>
}

impl Clone for Node {
	fn clone(self: &Node) -> Node {
		Node {
			innovation: self.innovation,
			node_type: self.node_type.copy(),
			activation_value: self.activation_value
		}
	}
}
impl Node {
	pub fn new() -> Node {
		Node {
			innovation: 0,
			node_type: Type::Hidden,
			activation_value: None
		}
	}

	pub fn give_input(self: &mut Node, input: f32) {
		self.activation_value = Some(self.node_type.activation(input));
	}
}

struct Network {
	node_genes: Vec<Node>,
	connection_genes: Vec<Connection>,
	num_sensors: usize,
	num_outputs: usize
}

impl Network {
	pub fn new(global: &mut Neat, sensors: usize, outputs: usize) -> Network {
		let mut nodes = Vec::with_capacity(sensors + outputs + 1);
		for i in 0..sensors + outputs + 1 {
			nodes.push(global.node_list[i].clone());
		}

		let mut network = Network {
			node_genes: nodes,
			connection_genes: Vec::new(),
			num_sensors: sensors,
			num_outputs: outputs
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
		
		output
	}

	fn evaluate_node(self: &mut Network, node: usize) -> f32 {
		let mut acc = 0.0;
		for i in 0..self.connection_genes.len() {
			if self.connection_genes[i].output == node {
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
		let to_node = rand::thread_rng().gen_range(self.num_sensors + 1..self.node_genes.len());

		let mut possible_from_nodes = vec![true; global.node_list.len()];

		for i in self.num_sensors + 1..self.num_outputs + self.num_sensors + 1 { // connection can't come from output node
			possible_from_nodes[i] = false;
		}

		let mut nodes_to_search = vec![to_node];
		while nodes_to_search.len() > 0 {
			let current_node = nodes_to_search.pop().unwrap();
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

		let from_node = from_nodes[rand::thread_rng().gen_range(0..from_nodes.len())];

		for connection_ptr in &global.connection_lookup[from_node] {
			if global.connection_list[*connection_ptr].output == to_node {
				for i in 0..self.connection_genes.len() {
					if self.connection_genes[i].innovation == *connection_ptr {
						self.connection_genes[i].weight = rand::thread_rng().gen::<f32>() * 2.0 - 1.0;
						return;
					}
				}
				self.connection_genes.push(Connection {
					innovation: *connection_ptr,
					input: from_node,
					output: to_node,
					weight: rand::thread_rng().gen::<f32>() * 2.0 - 1.0,
					enabled: true
				});
				return;
			}
		}
		
		self.connection_genes.push(Connection {
			innovation: global.connection_list.len(),
			input: from_node,
			output: to_node,
			weight: rand::thread_rng().gen::<f32>() * 2.0 - 1.0,
			enabled: true
		});
		global.connection_list.push(Connection {
			innovation: global.connection_list.len(),
			input: from_node,
			output: to_node,
			weight: 0.0,
			enabled: true
		});
	}
}
