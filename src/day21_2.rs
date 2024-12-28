const NUM_LEN: usize = 11;
#[derive(Clone,Copy,PartialEq,Debug)]
enum Num {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

#[derive(Clone,Copy,PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    A,
    Invalid,
}

fn char_to_num(num: char) -> Num {
    return match num {
        '0' => Num::Zero,
        '1' => Num::One,
        '2' => Num::Two,
        '3' => Num::Three,
        '4' => Num::Four,
        '5' => Num::Five,
        '6' => Num::Six,
        '7' => Num::Seven,
        '8' => Num::Eight,
        '9' => Num::Nine,
        'A' => Num::A,
        _ => panic!("Invalid number"),
    }
}

fn num_to_idx(num: Num) -> usize {
    return match num {
        Num::Zero => 0,
        Num::One => 1,
        Num::Two => 2,
        Num::Three => 3,
        Num::Four => 4,
        Num::Five => 5,
        Num::Six => 6,
        Num::Seven => 7,
        Num::Eight => 8,
        Num::Nine => 9,
        Num::A => 10
    }
}

const IDX_TO_NUM: [Num;NUM_LEN] = [Num::Zero, Num::One, Num::Two, Num::Three, Num::Four, Num::Five, Num::Six, Num::Seven, Num::Eight, Num::Nine, Num::A];

#[allow(dead_code)]
fn dir_to_idx(dir: Direction) -> usize {
    return match dir {
        Direction::Up => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 3,
        Direction::A => 4,
        _ => panic!("Invalid direction"),
    }
}

fn create_numpad() -> [[Direction; NUM_LEN]; NUM_LEN] {
    let mut ret: [[Direction; NUM_LEN]; NUM_LEN] = [[Direction::Invalid; NUM_LEN]; NUM_LEN];
    // A
    ret[num_to_idx(Num::A)][num_to_idx(Num::Zero)] = Direction::Left;
    ret[num_to_idx(Num::A)][num_to_idx(Num::Three)] = Direction::Up;
    // 0
    ret[num_to_idx(Num::Zero)][num_to_idx(Num::A)] = Direction::Right;
    ret[num_to_idx(Num::Zero)][num_to_idx(Num::Two)] = Direction::Up;
    // 1
    ret[num_to_idx(Num::One)][num_to_idx(Num::Four)] = Direction::Up;
    ret[num_to_idx(Num::One)][num_to_idx(Num::Two)] = Direction::Right;
    // 2
    ret[num_to_idx(Num::Two)][num_to_idx(Num::One)] = Direction::Left;
    ret[num_to_idx(Num::Two)][num_to_idx(Num::Three)] = Direction::Right;
    ret[num_to_idx(Num::Two)][num_to_idx(Num::Five)] = Direction::Up;
    ret[num_to_idx(Num::Two)][num_to_idx(Num::Zero)] = Direction::Down;
    // 3
    ret[num_to_idx(Num::Three)][num_to_idx(Num::Two)] = Direction::Left;
    ret[num_to_idx(Num::Three)][num_to_idx(Num::Six)] = Direction::Up;
    ret[num_to_idx(Num::Three)][num_to_idx(Num::A)] = Direction::Down;
    // 4
    ret[num_to_idx(Num::Four)][num_to_idx(Num::One)] = Direction::Down;
    ret[num_to_idx(Num::Four)][num_to_idx(Num::Five)] = Direction::Right;
    ret[num_to_idx(Num::Four)][num_to_idx(Num::Seven)] = Direction::Up;
    // 5
    ret[num_to_idx(Num::Five)][num_to_idx(Num::Two)] = Direction::Down;
    ret[num_to_idx(Num::Five)][num_to_idx(Num::Four)] = Direction::Left;
    ret[num_to_idx(Num::Five)][num_to_idx(Num::Six)] = Direction::Right;
    ret[num_to_idx(Num::Five)][num_to_idx(Num::Eight)] = Direction::Up;
    // 6
    ret[num_to_idx(Num::Six)][num_to_idx(Num::Three)] = Direction::Down;
    ret[num_to_idx(Num::Six)][num_to_idx(Num::Five)] = Direction::Left;
    ret[num_to_idx(Num::Six)][num_to_idx(Num::Nine)] = Direction::Up;
    // 7
    ret[num_to_idx(Num::Seven)][num_to_idx(Num::Four)] = Direction::Down;
    ret[num_to_idx(Num::Seven)][num_to_idx(Num::Eight)] = Direction::Right;
    // 8
    ret[num_to_idx(Num::Eight)][num_to_idx(Num::Five)] = Direction::Down;
    ret[num_to_idx(Num::Eight)][num_to_idx(Num::Seven)] = Direction::Left;
    ret[num_to_idx(Num::Eight)][num_to_idx(Num::Nine)] = Direction::Right;
    // 9
    ret[num_to_idx(Num::Nine)][num_to_idx(Num::Six)] = Direction::Down;
    ret[num_to_idx(Num::Nine)][num_to_idx(Num::Eight)] = Direction::Left;
    return ret;
}

fn parse_contents(contents: &String) -> Vec<Vec<Num>> {
    let mut ret = vec![];
    for line in contents.lines() {
        let mut nums = vec![];
        for c in line.chars() {
            nums.push(char_to_num(c));
        }
        ret.push(nums);
    }
    return ret;
}

fn dist_from_a_me(to: Direction) -> usize {
    // How many keystrokes to input a given direction
    // Starting position is A
    // Includes punching in A at the end
    // match to {
    //     Direction::Up => print!("^"), // Left -> A
    //     Direction::Down => print!("v"), // Left -> Down -> A
    //     Direction::Left => print!("<"), // Left -> Down -> Left -> A
    //     Direction::Right => print!(">"), // Down -> A
    //     Direction::A => print!("A"), // A
    //     _ => panic!("Invalid direction!"), // Invalid
    // }
    return 1;
}

fn dist_from_a_close(to: Direction) -> usize {
    // How many keystrokes to input a given direction
    // Starting position is A
    // Includes punching in A at the end
    return match to {
        Direction::Up => dist_from_a_me(Direction::Left) + dist_from_a_me(Direction::A), // Left -> A
        Direction::Down => dist_from_a_me(Direction::Left) + dist_from_a_me(Direction::Down) + dist_from_a_me(Direction::A), // Left -> Down -> A
        Direction::Left => dist_from_a_me(Direction::Left) + dist_from_a_me(Direction::Down) + dist_from_a_me(Direction::Left) + dist_from_a_me(Direction::A), // Left -> Down -> Left -> A
        Direction::Right => dist_from_a_me(Direction::Down) + dist_from_a_me(Direction::A), // Down -> A
        Direction::A => dist_from_a_me(Direction::A), // A
        _ => usize::MAX, // Invalid
    }
}

fn dist_from_a_far(dir: Direction) -> usize {
    // Get the number of keystrokes to go from A to another direction
    return match dir {
        Direction::A => dist_from_a_close(Direction::A),
        Direction::Up => dist_from_a_close(Direction::Left) + dist_from_a_close(Direction::A),
        Direction::Down => dist_from_a_close(Direction::Left) + dist_from_a_close(Direction::Down) + dist_from_a_close(Direction::A),
        Direction::Left => dist_from_a_close(Direction::Left) + dist_from_a_close(Direction::Down) + dist_from_a_close(Direction::Left) + dist_from_a_close(Direction::A),
        Direction::Right => dist_from_a_close(Direction::Down) + dist_from_a_close(Direction::A),
        _ => usize::MAX,
    }
}


fn get_num_neighbor_dist(from: usize, to: usize, numpad: [[Direction;NUM_LEN];NUM_LEN]) -> usize {
    // Get the distance from one number to another
    return dist_from_a_far(numpad[from][to]);
}

fn get_num_graph(numpad: [[Direction; NUM_LEN]; NUM_LEN]) -> Vec<Vec<usize>> {
    let each_num_fcn = |x: &[Direction;NUM_LEN]| {
        return (0..NUM_LEN).filter(|&i| x[i] != Direction::Invalid).collect::<Vec<usize>>();
    };
    // For each el in NUMPAD, collect all idxs that are not Direction::INVALID
    return numpad.iter().map(each_num_fcn).collect();
}

fn get_num_dists(numpad_graph: &Vec<Vec<usize>>, from: Num, numpad: [[Direction; NUM_LEN]; NUM_LEN]) -> [usize; NUM_LEN] {
    // Get the distance from one number to all others using Dijkstra
    let mut distances = core::array::from_fn(|_| (usize::MAX, Vec::new())); // (dist, path)
    let mut queue = vec![from];
    distances[num_to_idx(from)] = 1;
    // println!("FROM: {}: {:?}", num_to_idx(from), from);
    while let Some(u) = queue.pop() {
        let u_idx = num_to_idx(u);
        for &neighbor in numpad_graph[u_idx].iter() {
            let dist = get_num_neighbor_dist(u_idx, neighbor, numpad);
            if distances[u_idx] + dist < distances[neighbor] {
                distances[neighbor] = distances[u_idx] + dist;
                queue.push(IDX_TO_NUM[neighbor]);
            }
        }
    }
    return distances;
}



fn get_all_num_dists() -> [[usize;NUM_LEN];NUM_LEN] {
    let numpad  = create_numpad();
    let numpad_graph = get_num_graph(numpad);
    return core::array::from_fn(|x| get_num_dists(&numpad_graph, IDX_TO_NUM[x], numpad));
}

fn nums_to_usize(nums: &Vec<Num>) -> usize {
    let mut ret = 0;
    for &n in nums {
        if n != Num::A {
            ret = ret * 10 + num_to_idx(n);
        }
    }
    return ret;
}

fn part1(codes: &Vec<Vec<Num>>) -> usize {
    let all_num_dists = get_all_num_dists();
    let mut total_dist = 0;
    for code in codes {
        let mut dist = 0;
        let mut prev_num = Num::A;
        for &n in code {
            // println!("{:?} -> {:?}: {}", prev_num, n, all_num_dists[num_to_idx(prev_num)][num_to_idx(n)]);
            dist += all_num_dists[num_to_idx(prev_num)][num_to_idx(n)];
            prev_num = n;
        }
        println!("{}*{}", dist, nums_to_usize(code));
        total_dist += dist*nums_to_usize(code);
    }
    return total_dist;
}

pub fn fcn(contents: &String) {
    let codes = parse_contents(contents);
    let p1 = part1(&codes);
    println!("Part 1: {}", p1);
}