use regex::Regex;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
    BUF
}

fn to_opcode(s: u8) -> Opcode {
    return match s {
        0 => Opcode::ADV,
        1 => Opcode::BXL,
        2 => Opcode::BST,
        3 => Opcode::JNZ,
        4 => Opcode::BXC,
        5 => Opcode::OUT,
        6 => Opcode::BDV,
        7 => Opcode::CDV,
        _ => panic!("Invalid opcode")
    }
}

#[derive(Clone)]
struct ProgramState {
    program: Vec<(Opcode, u8)>,
    output: Vec<u8>,
    ptr: usize,
    reg: [u64; 3]
}

fn print_state(state: &ProgramState) {
    println!("Program: {:?}", state.program);
    println!("Output: {:?}", state.output);
    println!("Ptr: {:?}", state.ptr);
    println!("Reg: {:?}", state.reg);
}

fn read_contents(contents: &String) -> (ProgramState, Vec<u8>) {
    let mut regs = [0u64; 3];
    let mut program: Vec<(Opcode, u8)> = Vec::new();
    let mut program_u8: Vec<u8> = Vec::new();
    for (i,line) in contents.lines().enumerate() {
        if i < 3 {
            regs[i] = line.split(" ").last().unwrap().parse::<u64>().unwrap();
        }
        if i == 4 {
            program_u8 = line.split(" ").last().unwrap()
                .split(",").map(|x| x.parse().unwrap()).collect();
            // Match the program 1a,1b,2a,2b,3a,3b,... as [(1a,1b),(2a,2b),(3a,3b),...]
            let re = Regex::new(r"(\d),(\d)").unwrap();
            program = re.captures_iter(line).map(|c|
                return (to_opcode(c[1].parse().unwrap()), c[2].parse().unwrap())
            ).collect();
        }
    }
    program.insert(0,(Opcode::BUF,0));
    let state = ProgramState {
        program: program,
        output: Vec::new(),
        ptr: 1,
        reg: regs
    };
    return (state, program_u8);
}

#[inline]
fn get_combo(state: &ProgramState, operand: u8) -> u64 {
    if operand < 4 {
        return operand as u64;
    }
    return match operand {
        4 => state.reg[0],
        5 => state.reg[1],
        6 => state.reg[2],
        _ => panic!("Invalid operand")
    }
}

#[inline]
fn instruction_dv(state: &ProgramState, arg: u8) -> u64 {
    // DV: A >> arg (combo)
    let num = state.reg[0];
    let frac = get_combo(state, arg);
    // divide num by 2^frac using bitshift
    return num >> frac;
}

#[inline]
fn instruction_adv(state: &mut ProgramState, arg: u8) {
    // ADV: A >> arg (combo) -> A
    state.reg[0] = instruction_dv(state, arg);
}

#[inline]
fn instruction_bxl(state: &mut ProgramState, arg: u8) {
    // XOR: B ^ arg (literal) -> B
    state.reg[1] = state.reg[1] ^ (arg as u64);
}

#[inline]
fn instruction_bst(state: &mut ProgramState, arg: u8) {
    // BST: arg (combo) % 8 -> B
    state.reg[1] = get_combo(state, arg) % 8;
}

#[inline]
fn instruction_jnz(state: &mut ProgramState, arg: u8) {
    // JNZ: if A != 0, jump to arg (literal) - 1
    if state.reg[0] != 0 {
        state.ptr = (arg / 2) as usize;
    }
}

#[inline]
fn instruction_bxc(state: &mut ProgramState, _: u8) {
    // BXC: B ^ C -> B
    state.reg[1] = state.reg[1] ^ state.reg[2];
}

#[inline]
fn instruction_out(state: &mut ProgramState, arg: u8) {
    // OUT: arg (combo) -> output arg % 8
    let res = get_combo(state, arg) % 8;
    state.output.push(res as u8);
}

#[inline]
fn instruction_bdv(state: &mut ProgramState, arg: u8) {
    // BDV: A >> arg (combo) -> B
    state.reg[1] = instruction_dv(state, arg);
}

#[inline]
fn instruction_cdv(state: &mut ProgramState, arg: u8) {
    // CDV: A >> arg (combo) -> C
    state.reg[2] = instruction_dv(state, arg);
}

#[inline]
fn instruction(state: &mut ProgramState) -> bool {
    let (opcode, arg) = state.program[state.ptr];
    let mut out_change = false;
    match opcode {
        Opcode::ADV => instruction_adv(state, arg),
        Opcode::BXL => instruction_bxl(state, arg),
        Opcode::BST => instruction_bst(state, arg),
        Opcode::JNZ => instruction_jnz(state, arg),
        Opcode::BXC => instruction_bxc(state, arg),
        Opcode::OUT => {out_change = true; instruction_out(state, arg)},
        Opcode::BDV => instruction_bdv(state, arg),
        Opcode::CDV => instruction_cdv(state, arg),
        _ => panic!("Invalid opcode")
    }
    state.ptr += 1;
    return out_change;
}

fn execute(state: &mut ProgramState) {
    while state.ptr < state.program.len() {
        instruction(state);
    }
}

fn part1(state: &mut ProgramState) -> String {
    execute(state);
    return state.output.iter().map(|x| x.to_string()).join(",");
}

fn check_output(slice: &Vec<u8>, full_vec: &Vec<u8>) -> bool {
    if slice.len() > full_vec.len() {
        return false;
    }
    for j in 0..slice.len() {
        if slice[j] != full_vec[j] {
            return false;
        }
    }
    return true;
}

fn execute_match(state: &mut ProgramState, program: &Vec<u8>) {
    while state.ptr < state.program.len() {
        let out_change = instruction(state);
        if out_change {
            if !check_output(&state.output, program) {
                return;
            }
        }
    }
}

fn check_output_is_input(state: &ProgramState, reg_a: u64, program: &Vec<u8>) -> bool {
    let mut state_c = state.clone();
    state_c.reg[0] = reg_a;
    execute_match(&mut state_c, program);
    return state_c.output == *program;
}

fn part2_first(state: &ProgramState, program: &Vec<u8>) -> u64 {
    let mut reg_a = 0;
    while !check_output_is_input(state, reg_a, program) {
        reg_a += 1;
    }
    return reg_a;
}

fn hardcoded_program(a_init: u64, predicted_output: &Vec<u8>) -> bool {
    let mut a = a_init;
    let mut output_pointer = 0;
// # (BST, A), (BXL, 2), (CDV, B), (BXL, 3), (BXC, 3), (OUT, B), (ADV, 3), (JNZ, 0)
    while a > 0 {
        let b2 = (a % 8) ^ 2; // We take bottom three bits of A and negate second bit
        let c1 = a >> b2; // C = A >> B
        let b3 = b2 ^ c1; // B = B ^ C
        if predicted_output[output_pointer] == ((b3 % 8) as u8) {
            output_pointer += 1;
        } else {
            return false;
        }
        a = a >> 3;
    }
    return output_pointer == predicted_output.len();
}

fn part2_hardcoded(program: &Vec<u8>) -> u64 {
    let mut a = 0;
    while !hardcoded_program(a, program) {
        a += 1;
    }
    return a;
}

pub fn fcn(contents: &String) {
    let (state, program_u8) = read_contents(contents);
    let p1 = part1(&mut state.clone());
    println!("Part 1: {}", p1);
    println!("program_u8: {:?}", program_u8);
    print_state(&state);
    let p2 = part2_hardcoded(&program_u8);
    println!("Part 2: {}", p2);
    part2_first(&state, &program_u8);
}