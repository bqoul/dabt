use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Clone)]
struct Node {
	value: i128,
	left: Option<Box::<Node>>,
	right: Option<Box::<Node>>,
}

impl<'a> fmt::Display for Node {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		fn sort_rows(tree: Option<Box<Node>>, node_origin: NodeOrigin, rows: &mut Vec<String>, row_pos: usize, spaces: usize) {
			if let Some(node) = tree {
				/*
					ascii symbols that are used for displaying 
					tree arms have length of 3 instead of 1, so
					'decrement' is how much we need to subtract from row.len() 
					to get the actual length of the row
				*/
				let mut decrement: usize = 5;

				let (mut back_arm, mut front_arm) = (' ', ' ');

				match node_origin {
					NodeOrigin::Right => back_arm = '┌',
					NodeOrigin::Left  => back_arm = '└',
					NodeOrigin::Main  => decrement -= 2,
				}

				match (node.left.is_none(), node.right.is_none()) {
					(false, false) => front_arm = '┤', 	// left and right nodes
					(false, true)  => front_arm = '┐',  // only left node
					(true, false)  => front_arm = '┘',  // only right node
					(true, true)   => decrement -= 3,   // no nodes
				}

				let row = format!("{}{}[{}]{}", " ".repeat(spaces), back_arm, node.value.to_string(), front_arm);
				//i wrote it, it works, but i have no idea how and why
				if let NodeOrigin::Left = node_origin {
					rows.insert(row_pos + 1, row.clone());
				} else {
					rows.insert(row_pos, row.clone());
				}

				sort_rows(
					node.right, 
					NodeOrigin::Right, 
					rows, 
					rows.iter().position(|r| *r == row).unwrap(), 
					row.len() - decrement
				);

				sort_rows(
					node.left, 
					NodeOrigin::Left, 
					rows, 
					rows.iter().position(|l| *l == row).unwrap(), 
					row.len() - decrement
				);
			}
		}

		let mut rows: Vec<String> = Vec::new();
		sort_rows(Some(Box::new(self.clone())), NodeOrigin::Main, &mut rows, 0, 0);

		//filling emply space between arms with '│'
		for r in 0..rows.clone().len() - 1 {
			let cur_row: Vec<char> = rows[r].chars().collect();
			let mut bot_row: Vec<char> = rows[r+1].chars().collect();

			for c in 0..cur_row.len() {
				if cur_row[c] == '┤' || cur_row[c] == '┌' || cur_row[c] == '│' {
					if bot_row[c] == ' ' { bot_row[c] = '│'; }
				}
			}
			rows[r + 1] = bot_row.iter().collect();
		}

		return write!(formatter, "\n{}", rows.join("\n"));
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 { 
		println!("ERROR: wrong amount of arguments, expected at least 1");
		return;
	}

	for i in 1..args.len() {
		let file_name: Vec<&str> = args[i].split(".").collect();
		if file_name.len() != 2 {
			println!("ERROR: no file extension specified");
			return;
		} else if file_name[1] != "json" {
			println!("ERROR: wrong file extension, expected 'json'");
			return;
		}

		match File::open(format!("{}", args[i])) {
			Ok(mut file) => {
				let mut raw_str = String::new();

				match file.read_to_string(&mut raw_str) {
					Ok(_) => {
						let tree: Node = serde_json::from_str(&raw_str).unwrap();
						if args.len() != 2 { println!("\n{}:{}\n", args[i], tree); }
						else { println!("{}\n", tree); }
					},

					Err(why) => println!("ERROR: failed to read the file\n{}", why),
				}
			},

			Err(why) => println!("ERROR: failed to open the file\n{}", why),
		}
	}
}