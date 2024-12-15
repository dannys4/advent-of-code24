fn parse_contents(contents: &String) -> Vec<u8> {
    return contents
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
}

fn create_memory(numbers: &Vec<u8>) -> Vec<u32> {
    let total_memory = numbers.into_iter().fold(0usize, |acc, x| acc + *x as usize);
    // Assert total memory can fit into a u32
    assert!(
        total_memory <= u32::MAX as usize,
        "Total memory needed {} exceeds u32::MAX {}",
        total_memory,
        u32::MAX
    );
    let mut alloc_vec = Vec::<u32>::with_capacity(total_memory);
    let mut id = 0;
    for (idx, num) in numbers.iter().enumerate() {
        for _ in 0..*num {
            if idx % 2 == 0 {
                alloc_vec.push(id);
            } else {
                alloc_vec.push(u32::MAX);
            }
        }
        if idx % 2 == 0 {
            id += 1;
        }
    }
    return alloc_vec;
}

fn defrag_memory(memory: &mut Vec<u32>) {
    let mut rev_idx = memory.len() - 1;
    let mut idx = 0;
    loop {
        if idx >= rev_idx {
            break;
        }
        while idx < rev_idx && memory[idx] != u32::MAX {
            idx += 1;
        }
        while idx < rev_idx && memory[idx] == u32::MAX {
            while idx < rev_idx && memory[rev_idx] == u32::MAX {
                rev_idx -= 1;
            }

            if idx >= rev_idx {
                break;
            }

            memory[idx] = memory[rev_idx];
            memory[rev_idx] = u32::MAX;
            idx += 1;
        }
    }
}

fn part1(numbers: &Vec<u8>) -> usize {
    let mut memory = create_memory(numbers);
    defrag_memory(&mut memory);
    let (mut sum, mut idx) = (0, 0);
    while idx < memory.len() && memory[idx] != u32::MAX {
        sum += (memory[idx] as usize) * idx;
        idx += 1;
    }
    return sum;
}

// Use DLLNode for "linked" list
struct DLLNode {
    next_idx: usize,
    prev_idx: usize,
    id: u32, // id == u32::MAX for free memory
    size: u8,
}

// Use Vec<DLLNode> as doubly linked list
struct DLL {
    data: Vec<DLLNode>,
    head_idx: usize,
    tail_idx: usize,
}

impl DLL {
    fn new() -> DLL {
        return DLL {
            data: Vec::new(),
            head_idx: 0,
            tail_idx: 0,
        };
    }

    fn push_back(&mut self, id: u32, size: u8) {
        let prev_idx = self.tail_idx;
        let next_idx = self.data.len();
        self.data.push(DLLNode {
            next_idx: usize::MAX,
            prev_idx,
            id,
            size,
        });
        self.data[prev_idx].next_idx = next_idx;
        self.tail_idx = next_idx;
    }

    fn push_back_phys(&mut self, id: u32, size: u8, prev_idx: usize, next_idx: usize) -> usize {
        // put node at the end of the list
        self.data.push(DLLNode {
            next_idx,
            prev_idx,
            id,
            size,
        });
        // update the next_idx of the previous node and the prev_idx of the next node
        self.data[prev_idx].next_idx = self.data.len() - 1;
        self.data[next_idx].prev_idx = self.data.len() - 1;
        return self.data.len() - 1;
    }

    fn print(&self) {
        let mut idx = self.head_idx;
        while idx != usize::MAX {
            let node = &self.data[idx];
            println!("Node: id: {}, size: {}", node.id, node.size);
            idx = node.next_idx;
        }
    }
    fn swap_into(&mut self, node_curr_idx: usize, node_dest_idx: usize) {
        let curr_next = self.data[node_curr_idx].next_idx;
        // Check that node_dest_idx and node_curr_idx.next is free memory
        assert!(self.data[node_dest_idx].id == u32::MAX && self.data[curr_next].id == u32::MAX, "node_dest_idx: {}, node_curr_idx.next: {}", self.data[node_dest_idx].id, self.data[curr_next].id);
        // Check that node_curr_idx is not free memory
        assert!(self.data[node_curr_idx].id != u32::MAX);
        // Check that node_dest_idx.size >= node_curr_idx.size
        assert!(self.data[node_dest_idx].size >= self.data[node_curr_idx].size);
        // Check node_dest_idx > node_curr_idx
        assert!(node_dest_idx > node_curr_idx);
        // Place node_curr_idx into node_dest_idx, making node_curr_idx.next free memory larger by node_curr_idx.size
        // And creating a new node with size = node_dest_idx.size - node_curr_idx.size placed between curr_node and dest_node.next
        let curr_sz = self.data[node_curr_idx].size;
        let dest_sz = self.data[node_dest_idx].size;
        let dest_prev = self.data[node_dest_idx].prev_idx;
        let dest_next = self.data[node_dest_idx].next_idx;
        let new_size = dest_sz - curr_sz;
        if new_size == 0 {
            self.data[node_curr_idx].prev_idx = dest_prev;
            self.data[node_curr_idx].next_idx = dest_next;
            self.data[dest_next].prev_idx = node_curr_idx;
            self.data[dest_prev].next_idx = node_curr_idx;
            self.data[curr_next].size += curr_sz;
        } else {
            let new_idx = self.push_back_phys(u32::MAX, new_size, node_curr_idx, dest_next);
            self.data[node_curr_idx].prev_idx = dest_prev;
            self.data[node_curr_idx].next_idx = new_idx;
            self.data[dest_next].prev_idx = new_idx;
            self.data[dest_prev].next_idx = node_curr_idx;
            self.data[curr_next].size += curr_sz;
        }
    }
}

fn part2(numbers: &Vec<u8>) -> usize {
    let mut memory: DLL = DLL::new();
    let mut id = 0;
    // Create memory in a linked list
    for (idx, num) in numbers.iter().enumerate() {
        if idx % 2 == 0 {
            memory.push_back(id, *num);
            id += 1;
        } else {
            memory.push_back(u32::MAX, *num);
        }
    }
    memory.print();
    // Defrag memory using _whole blocks_
    // Requires swaps from beginning to end
    // Also defrag memory on the end of the list on the fly (consolidate free memory)
    // let mut node = memory.into_iter();
    // let mut rev_node = memory.into_iter().rev();
    let mut node_idx = memory.head_idx;
    let mut rev_node_idx = memory.tail_idx;
    while node_idx != usize::MAX {
        let node = &memory.data[node_idx];
        if node.id == u32::MAX {
            // Find a node with id != u32::MAX
            while rev_node_idx != usize::MAX && (memory.data[rev_node_idx].id == u32::MAX || memory.data[rev_node_idx].size < node.size) {
                println!("rev_node_idx: {}", rev_node_idx);
                rev_node_idx = memory.data[rev_node_idx].prev_idx;
            }
            if rev_node_idx == usize::MAX {
                break;
            }
            memory.swap_into( rev_node_idx, node_idx);
        }
        node_idx = memory.data[node_idx].next_idx;
    }
    return 0;
}

pub fn fcn(contents: &String) {
    // First convert every character to a number
    let numbers = parse_contents(contents);
    let p1 = part1(&numbers);
    println!("Part 1: {}", p1);
    let p2 = part2(&numbers);
    println!("Part 2: {}", p2);
}
