use serde::{Deserialize};
use std::{
	fmt, 
	env, 
	fs::File,
	io::prelude::*,
};

enum NodeOrigin {
	Main,
	Right,
	Left,
}

#[derive(Deserialize, Clone)]
struct Node {
	value: i128,
	left: Option<Box::<Node>>,
	right: Option<Box::<Node>>,
}

impl<'a> fmt::Display for Node {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		fn sort_display(tree: Option<Box<Node>>, display: &mut Vec<String>, origin: NodeOrigin, spaces: usize) {
			if let Some(node) = tree {
				let mut back = ' ';
				match origin {
					NodeOrigin::Right => back = '┌',
					NodeOrigin::Left  => back = '└',
					NodeOrigin::Main  => {},
				}

				let mut front = ' ';
				match (node.left.is_none(), node.right.is_none()) {
					(false, false) => front = '┤', 	// left and right nodes
					(false, true)  => front = '┐',  // only left node
					(true, false)  => front = '┘',  // only right node
					(true, true)   => {},           // no nodes
				}

				let mut row: Vec<char> = format!("{}{}[{}]{}", " ".repeat(spaces), back, node.value, front).chars().collect();
				//adding rows to 'display' from right to left
				sort_display(node.right, display, NodeOrigin::Right, spaces + node.value.to_string().len() + 3);
		
				if let Some(last_row) = display.last() {
					const EDGES: [char; 4] = ['┤', '┌', '│', '┐'];
					//formatting the row for correct tree display
					for (p, c) in last_row.chars().enumerate() {
						if EDGES.contains(&c) && row[p] == ' ' {
							row[p] = '│';
						}
					}
				}
				
				display.push(row.iter().collect());

				sort_display(node.left, display, NodeOrigin::Left, spaces + node.value.to_string().len() + 3);
			}
		}

		let mut display = Vec::new();
		sort_display(Some(Box::new(self.clone())), &mut display, NodeOrigin::Main, 0);
		return write!(formatter, "{}", display.join("\n"));
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 { 
		println!("ERROR: wrong amount of arguments, expected at least 1");
		return;
	}

	for i in 1..args.len() {
		let mut tag = String::new();
		let mut file_name: Vec<&str> = args[i].split(".").collect();

		if file_name.len() != 2 {
			file_name.push("json");
		} else if file_name[1] != "json" {
			println!("{}\nERROR: wrong file extension, expected 'json'", tag);
			continue;
		}

		if args.len() != 2 { tag = format!("[{}]\n", file_name.join(".")); }

		match File::open(format!("{}", file_name.join("."))) {
			Ok(mut file) => {
				let mut raw_str = String::new();

				match file.read_to_string(&mut raw_str) {
					Ok(_) => {
						let tree: Node = match serde_json::from_str(&raw_str) {
							Ok(tree) => tree,
							Err(why) => {
								println!("{}ERROR: failed file deserialization\n{}\n", tag, why);
								continue;
							}
						};

						println!("\n{}{}\n", tag, tree);
					},

					Err(why) => println!("{}ERROR: failed to read the file\n{}\n", tag, why),
				}
			},

			Err(why) => println!("{}ERROR: failed to open the file\n{}\n", tag, why),
		}
	}
}