
fn find_min_dist(queue: &Vec<usize>, dist: &Vec<usize>) -> usize {
    let mut min_q_idx = queue.len() - 1;
    let mut min = dist[queue[min_q_idx]];
    for (node_idx, &node) in queue.iter().enumerate() {
        if dist[node] < min {
            min = dist[node];
            min_q_idx = node_idx;
        }
    }
    return min_q_idx;
}

fn dijkstra(graph: Vec<Vec<usize>>, start: usize, end: usize) -> usize {
    // Graph: CSR representation of adjacency matrix. All weights are 1
    let mut dist = vec![usize::MAX; graph.len()];
    let mut prev = vec![usize::MAX; graph.len()];
    dist[start] = 0;
    let mut queue = (0..graph.len()).collect::<Vec<usize>>();
    while !queue.is_empty() && dist[end] == usize::MAX {
        let min_q_idx = find_min_dist(&queue, &dist);
        let u = queue.remove(min_q_idx);
        if dist[u] == usize::MAX {
            break;
        }
        for &v in graph[u].iter() {
            let alt = dist[u] + 1;
            if alt < dist[v] {
                dist[v] = alt;
                prev[v] = u;
            }
        }
    }
    return dist[end];
}

fn linear_index(idx: (usize, usize), width: usize) -> usize {
    return idx.0 * width + idx.1;
}

fn get_neighbors(idx: (usize, usize), size: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if idx.0 > 0 {
        neighbors.push((idx.0 - 1, idx.1));
    }
    if idx.0 < size.0 - 1 {
        neighbors.push((idx.0 + 1, idx.1));
    }
    if idx.1 > 0 {
        neighbors.push((idx.0, idx.1 - 1));
    }
    if idx.1 < size.1 - 1 {
        neighbors.push((idx.0, idx.1 + 1));
    }
    return neighbors;
}

fn parse_contents(contents: &String) -> Vec<(usize,usize)> {
    // Parse contents as (i,j) coordinates
    let mut coords = Vec::new();
    for line in contents.lines() {
        let mut coord = line.split(",");
        let i = coord.next().unwrap().parse::<usize>().unwrap();
        let j = coord.next().unwrap().parse::<usize>().unwrap();
        coords.push((i,j));
    }
    return coords;
}

fn create_graph(obstacles: &[(usize,usize)], size: (usize, usize)) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; size.0 * size.1];
    for i in 0..size.0 {
        for j in 0..size.1 {
            let idx = (j,i);
            if obstacles.contains(&idx) {
                continue;
            }
            let u = linear_index(idx, size.1);
            let neighbors = get_neighbors(idx, size);
            for neighbor in neighbors {
                if !obstacles.contains(&neighbor) {
                    let v = linear_index(neighbor, size.1);
                    graph[u].push(v);
                }
            }
        }
    }
    return graph;
}

#[allow(dead_code)]
fn print_maze(graph: &Vec<Vec<usize>>, size: (usize, usize)) {
    let width = size.1;
    let mut chars = vec![vec!['.'; size.1]; size.0];
    for i in 0..size.0 {
        for j in 0..size.1 {
            let idx = (j,i);
            let u = linear_index(idx, width);
            if graph[u].len() == 0 {
                chars[i][j] = '#';
            }
        }
    }
    for i in 0..size.0 {
        for j in 0..size.1 {
            print!("{}", chars[i][j]);
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_graph(graph: &Vec<Vec<usize>>, size: (usize, usize)) {
    let width = size.1;
    for (idx, neighbors) in graph.iter().enumerate() {
        let i = idx / width;
        let j = idx % width;
        print!("({},{}): ", i, j);
        for &neighbor in neighbors.iter() {
            let i_n = neighbor / width;
            let j_n = neighbor % width;
            print!("({},{}), ", i_n, j_n);
        }
        println!();
    }
}

const SIZE: (usize, usize) = (71,71);
const N_OBSTACLES_P1: usize = 1024;

fn part1(obstacles: &Vec<(usize,usize)>)->usize {
    let start = (0,0);
    let end = (SIZE.0-1,SIZE.1-1);
    let graph = create_graph(&obstacles[..N_OBSTACLES_P1], SIZE);
    // print_maze(&graph, SIZE);
    let u_start = linear_index(start, SIZE.1);
    let u_end = linear_index(end, SIZE.1);
    return dijkstra(graph, u_start, u_end);
}

fn part2(obstacles: &Vec<(usize,usize)>)->(usize,usize) {
    let start = (0,0);
    let end = (SIZE.0-1,SIZE.1-1);
    let u_start = linear_index(start, SIZE.1);
    let u_end = linear_index(end, SIZE.1);

    // Binary search for the first obstacle that makes the path unreachable
    let mut lo = N_OBSTACLES_P1;
    let mut hi = obstacles.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        let graph = create_graph(&obstacles[..mid], SIZE);
        let dist = dijkstra(graph, u_start, u_end);
        if dist == usize::MAX {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    return obstacles[lo-1];
}

pub fn fcn(contents: &String) {
    let obstacles = parse_contents(contents);
    let p1 = part1(&obstacles);
    println!("Part 1: {}", p1);
    let p2 = part2(&obstacles);
    println!("Part 2: {},{}", p2.0, p2.1);
}