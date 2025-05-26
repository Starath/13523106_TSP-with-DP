use std::vec;

pub fn solve_tsp(graph: &Vec<Vec<i32>>, start_node: usize) -> Option<(i32, Vec<usize>)> {
	// Backward DP
	let n = graph.len();

	if n == 0 {
		return Some((0, Vec::new()));
	}

	if graph.iter().any(|row| row.len() != n) {
		eprintln!("Error: Graph is not a valid adjacency matrix (rows have inconsistent lengths).");
		return None;
	}
	if start_node >= n {
		eprintln!("Error: Start node is out of bounds.");
		return None;
	}

	if n == 1 {
		let cost = if !graph[start_node].is_empty() && graph[start_node][start_node] != i32::MAX {
			0
		} else {
			0 
		};
		return Some((cost, vec![start_node, start_node]));
	}

	let mut dp = vec![vec![i32::MAX; n]; 1 << n];
	let mut parent = vec![vec![0; n]; 1 << n];

	dp[1 << start_node][start_node] = 0; // Base Case

	for mask in 1..(1 << n) {
		for i in 0..n { 
			if (mask & (1 << i)) == 0 { 
				continue;
			}

			if mask == (1 << i) {
				continue; 
			}

			let prev_mask: usize = mask ^ (1 << i);
			if prev_mask == 0 {
				continue;
			}
			
			for j in 0..n { 
				if (prev_mask & (1 << j)) == 0 { 
					continue;
				}

				if dp[prev_mask][j] != i32::MAX && graph[j][i] != i32::MAX {
					let new_cost = dp[prev_mask][j].saturating_add(graph[j][i]);
					if new_cost < dp[mask][i] {
						dp[mask][i] = new_cost;
						parent[mask][i] = j;
					}
				}
			}
		}
	}

	let final_mask = (1 << n) - 1;
	let mut min_total_cost = i32::MAX;
	let mut last_city_in_path = start_node;

	for i in 0..n {
		if i == start_node && n > 1 { 
			continue;
		}

		if dp[final_mask][i] != i32::MAX && graph[i][start_node] != i32::MAX {
			let current_total_cost = dp[final_mask][i].saturating_add(graph[i][start_node]);
			if current_total_cost < min_total_cost {
				min_total_cost = current_total_cost;
				last_city_in_path = i;
			}
		}
	}

	if min_total_cost == i32::MAX {
		return None; 
	}

	let mut path = Vec::with_capacity(n + 1);
	let mut current_mask = final_mask;
	let mut current_city = last_city_in_path;

	for _ in 0..n {
		path.push(current_city);
		if current_mask == (1 << start_node) && current_city == start_node {
			break; 
		}
		if current_mask == 0 && n > 0 { return None; }


		let prev_city = parent[current_mask][current_city];
		current_mask ^= 1 << current_city;
		current_city = prev_city;

		if path.len() > n { 
			eprintln!("Error: Path reconstruction exceeded n cities.");
			return None;
		}
	}
	
	path.reverse(); 
	path.push(start_node);

	Some((min_total_cost, path))
}

// Read user input for an integer, with error handling
fn read_int(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Input not valid. Please enter a valid integer."),
        }
    }
}

// Read user input for a cost, allowing "inf" for infinite cost
fn read_cost(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("inf") {
            return i32::MAX;
        }
        match trimmed_input.parse() {
            Ok(num) => return num,
            Err(_) => println!("Input not valid\n Please enter a valid integer or 'inf' for infinite cost"),
        }
    }
}


fn main() {
    println!("--- Program Penyelesaian TSP dengan Dynamic Programming ---");

    let num_cities = read_int("Masukkan jumlah kota:");

    if num_cities == 0 {
        println!("Jumlah kota tidak boleh nol. Program berhenti.");
        return;
    }

    let mut user_graph: Vec<Vec<i32>> = Vec::with_capacity(num_cities);

    println!("\nMasukkan matriks adjacency (biaya antar kota):");
    println!("(Gunakan 'inf' untuk merepresentasikan tidak ada jalur langsung / biaya tak hingga)");

    for i in 0..num_cities {
        let mut row: Vec<i32> = Vec::with_capacity(num_cities);
        println!("Biaya dari kota {} ke kota lain:", i);
        for j in 0..num_cities {
            if i == j {
                println!("  Biaya dari kota {} ke kota {} (otomatis 0): 0", i, j);
                row.push(0);
            } else {
                let cost_prompt = format!("  Biaya dari kota {} ke kota {}: ", i, j);
                row.push(read_cost(&cost_prompt));
            }
        }
        user_graph.push(row);
    }

    let mut start_node_user: usize;
    loop {
        start_node_user = read_int("\nMasukkan kota awal (indeks 0-based):");
        if start_node_user < num_cities {
            break;
        } else {
            println!("Kota awal tidak valid. Harus kurang dari jumlah kota ({}).", num_cities);
        }
    }
    

    println!("\nGraf yang dimasukkan:");
    for row in &user_graph {
        for &val in row {
            if val == i32::MAX {
                print!("{:>5} ", "inf");
            } else {
                print!("{:>5} ", val);
            }
        }
        println!();
    }


    println!("\nMencari solusi TSP untuk graf pengguna...");
    match solve_tsp(&user_graph, start_node_user) {
        Some((cost, path)) => {
            println!("  Biaya Minimum: {}", cost);
            println!("  Jalur Optimal: {:?}", path);
        }
        None => {
            println!("  Tidak ditemukan solusi untuk graf yang dimasukkan.");
        }
    }
}
