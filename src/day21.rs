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

#[derive(Clone,Copy,PartialEq,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    A,
    Invalid,
}

const DIR_LEN: usize = 5;
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

fn get_num_graph(numpad: &[[Direction; NUM_LEN]; NUM_LEN]) -> Vec<Vec<usize>> {
    let each_num_fcn = |x: &[Direction;NUM_LEN]| {
        return (0..NUM_LEN).filter(|&i| x[i] != Direction::Invalid).collect::<Vec<usize>>();
    };
    // For each el in NUMPAD, collect all idxs that are not Direction::INVALID
    return numpad.iter().map(each_num_fcn).collect();
}

fn get_num_dists(numpad: &[[Direction; NUM_LEN]; NUM_LEN], numpad_graph: &Vec<Vec<usize>>, from: Num) -> [Vec<Direction>; NUM_LEN] {
    // Get the distance from one number to all others using Dijkstra
    let mut distances = core::array::from_fn(|_|  vec![Direction::Invalid; NUM_LEN+1]); // (path)
    let mut queue = vec![from];
    distances[num_to_idx(from)] = Vec::new();
    // println!("FROM: {}: {:?}", num_to_idx(from), from);
    while let Some(u) = queue.pop() {
        let u_idx = num_to_idx(u);
        let u_hist = distances[u_idx].clone();
        for &neighbor in numpad_graph[u_idx].iter() {
            if u_hist.len() + 1 < distances[neighbor].len() {
                let mut new_hist = u_hist.clone();
                new_hist.push(numpad[u_idx][neighbor]);
                distances[neighbor] = new_hist;
                queue.push(IDX_TO_NUM[neighbor]);
            }
        }
    }
    return distances;
}

fn get_all_num_dists() -> [[Vec<Direction>;NUM_LEN];NUM_LEN] {
    let numpad  = create_numpad();
    let numpad_graph = get_num_graph(&numpad);
    let mut all_num_dists = core::array::from_fn(|_| core::array::from_fn(|_| Vec::new()));
    for i in 0..NUM_LEN {
        let num = IDX_TO_NUM[i];
        let num_dists = get_num_dists(&numpad, &numpad_graph, num);
        for j in 0..NUM_LEN {
            println!("{:?} -> {:?}: {}, {:?}", num, IDX_TO_NUM[j], num_dists[j].len(), num_dists[j]);
            all_num_dists[i][j] = num_dists[j].clone();
        }
    }
    return all_num_dists;
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

fn print_dir(dir: Direction) {
    match dir {
        Direction::Up => print!("^"),
        Direction::Down => print!("v"),
        Direction::Left => print!("<"),
        Direction::Right => print!(">"),
        Direction::A => print!("A"),
        _ => print!("X"),
    }
}

fn human_fixed_point(dir: Direction) -> Vec<Direction> {
    print_dir(dir);
    return vec![dir];
}

fn dirpad_path(from: Direction, to: Direction) -> Vec<Direction> {
    let ret =  match from {
        Direction::A => match to {
            Direction::Up => vec![Direction::Left],
            Direction::Down => vec![Direction::Left, Direction::Down],
            Direction::Left => vec![Direction::Down, Direction::Left, Direction::Left],
            Direction::Right => vec![Direction::Down],
            Direction::A => vec![],
            _ => vec![Direction::Invalid],
        },
        Direction::Up => match to {
            Direction::Up => vec![],
            Direction::Down => vec![Direction::Down],
            Direction::Left => vec![Direction::Down, Direction::Left],
            Direction::Right => vec![Direction::Down, Direction::Right],
            Direction::A => vec![Direction::Right],
            _ => vec![Direction::Invalid],
        },
        Direction::Down => match to {
            Direction::Up => vec![Direction::Up],
            Direction::Down => vec![],
            Direction::Left => vec![Direction::Left],
            Direction::Right => vec![Direction::Right],
            Direction::A => vec![Direction::Up, Direction::Right],
            _ => vec![Direction::Invalid],
        },
        Direction::Left => match to {
            Direction::Up => vec![Direction::Right, Direction::Up],
            Direction::Down => vec![Direction::Right],
            Direction::Left => vec![],
            Direction::Right => vec![Direction::Right, Direction::Right],
            Direction::A => vec![Direction::Right, Direction::Right, Direction::Up],
            _ => vec![Direction::Invalid],
        },
        Direction::Right => match to {
            Direction::Up => vec![Direction::Left, Direction::Up],
            Direction::Down => vec![Direction::Left],
            Direction::Left => vec![Direction::Left, Direction::Left],
            Direction::Right => vec![],
            Direction::A => vec![Direction::Up],
            _ => vec![Direction::Invalid],
        },
        _ => vec![Direction::Invalid],
    };
    return ret;
}

fn fixed_point(from: Direction, to: Direction, layer: usize) -> Vec<Direction> {
    if layer == 0 {
        return vec![to];
    } else {
        let direct_path = dirpad_path(from, to);
        let mut prev_dir = from;
        let mut ret = vec![];
        for &dir in &direct_path {
            let mut path = fixed_point(prev_dir, dir, layer - 1);
            ret.append(&mut path);
            prev_dir = dir;
        }
        return ret;
    }
}

fn part1(codes: &Vec<Vec<Num>>) -> usize {
    const START_LAYER: usize = 2;
    let all_num_dists = get_all_num_dists();
    let mut total_dist = 0;
    println!();
    for code in codes {
        let mut dist = 0;
        let mut prev_num = Num::A;
        let mut prev_dir = Direction::A;
        for &n in code {
            print!("{:?} -> {:?}: ", prev_num, n);
            let mut dirs = all_num_dists[num_to_idx(prev_num)][num_to_idx(n)].clone();
            print!("{:?} || ", dirs);
            for &dir in &dirs {
                let mut path = fixed_point(prev_dir, dir, START_LAYER);
                for &p in &path {
                    print_dir(p);
                }
                dist += path.len();
                prev_dir = dir;
            }
            prev_num = n;
            println!();
        }
        println!("\n{}*{}", dist, nums_to_usize(code));
        total_dist += dist*nums_to_usize(code);
    }
    return total_dist;
}

pub fn fcn(contents: &String) {
    let codes = parse_contents(contents);
    let p1 = part1(&codes);
    println!("Part 1: {}", p1);
}