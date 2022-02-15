// use std::
fn main() {
    unreachable!()
}

fn parse_inst(mut pos: usize, tokens: &[&str]) -> (Instruction, usize) {
    let mut t_branch: Vec<Instruction> = vec![];
    let mut f_branch: Vec<Instruction> = vec![];
    println!(
        "parse_inst() t_branch={:?}; f_branch={:?}",
        t_branch, f_branch
    );
    println!(
        "parse_inst(): pos: {}; tokens[pos]: {}; tokens: {:?}",
        pos, tokens[pos], tokens
    );

    let op_v = tokens[pos].split(".").collect::<Vec<_>>();
    let inst = match op_v[0] {
        "add" => {
            pos += 1;
            Instruction::Add
        }
        "push" => {
            pos += 1;
            let v = op_v[1].parse::<u64>().unwrap();
            Instruction::Push(v)
        }
        "mul" => {
            pos += 1;
            Instruction::Mul
        }
        "if" | "else" => {
            pos += 1;
            while tokens[pos] != "endif" {
                println!(
                    "parse_inst(): if: before: pos: {}; tokens[pos]: {}; pos: {}",
                    pos, tokens[pos], pos
                );

                let (t_inst, next_pos) = parse_inst(pos, tokens);
                if op_v[0] == "if" {
                    t_branch.push(t_inst);
                    println!("parse_inst(): if: t_branch: {:?}; pos: {}", t_branch, pos);
                } else {
                    f_branch.push(t_inst);
                    println!("parse_inst(): else: f_branch: {:?}; pos: {}", f_branch, pos);
                }

                println!(
                    "parse_inst(): if: loop: pos: {}; tokens[next_pos]: {}; next_pos: {}",
                    pos, tokens[next_pos], next_pos
                );

                pos = next_pos;

                // if tokens[pos] == "endif" {
                //     println!(
                //         "parse_inst(): if: break: pos: {}; tokens[next_pos]: {}; next_pos: {}",
                //         pos, tokens[next_pos], next_pos
                //     );
                //     break;
                // };
            }

            pos += 1;
            println!(
                "parse_inst(): if: returning: pos: {}; tokens[pos]: {}",
                pos, tokens[pos]
            );
            Instruction::IfElse { t_branch, f_branch }
        }
        _ => panic!("pos: {}, inst: {:?}, tokens: {:?}", pos, op_v, tokens),
    };

    println!("parse_inst(): returning: pos: {}; inst: {:?}", pos, inst);
    return (inst, pos);
}

pub fn parse(source: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    let tokens = source.split_whitespace().collect::<Vec<_>>();

    let mut pos: usize = 0;
    // while tokens[pos] != "end" {
    loop {
        if tokens[pos] == "begin" {
            pos += 1;
            continue;
        }

        println!(
            "parse(): parser_inst() pos={}; tokens[pos]={}",
            pos, tokens[pos]
        );
        let (inst, next_pos) = parse_inst(pos, &tokens);

        println!(
            "parse(): after parser_inst() pos={}; tokens[next_pos]={}; next_pos={}",
            pos, tokens[next_pos], next_pos
        );

        instructions.push(inst);
        pos = next_pos;

        if tokens[pos] == "end" {
            break;
        }
    }

    instructions
}

// INSTRUCTIONS
// =================================================================================

#[derive(Debug)]
pub enum Instruction {
    Push(u64),
    Add,
    Mul,
    IfElse {
        t_branch: Vec<Instruction>,
        f_branch: Vec<Instruction>,
    },
}

// TESTS
// =================================================================================

#[test]
fn test_simple() {
    let program = parse("begin push.1 push.2 add end");
    assert!(matches!(program[0], Instruction::Push(1)));
    assert!(matches!(program[1], Instruction::Push(2)));
    assert!(matches!(program[2], Instruction::Add));
}

#[test]
fn test_if() {
    let program = parse("begin push.1 if.true push.3 add endif end");
    println!("{:?}", program);
    assert!(matches!(program[0], Instruction::Push(1)));
    match &program[1] {
        Instruction::IfElse { t_branch, f_branch } => {
            assert!(matches!(t_branch[0], Instruction::Push(3)));
            assert!(matches!(t_branch[1], Instruction::Add));
            assert_eq!(f_branch.len(), 0);
        }
        _ => panic!("unexpected instruction: {:?}", program[1]),
    }
}

#[test]
fn test_if_else() {
    let program = parse("begin push.1 push.2 add if.true push.3 else push.4 endif mul end");
    println!("{:?}", program);
    assert!(matches!(program[0], Instruction::Push(1)));
    assert!(matches!(program[1], Instruction::Push(2)));
    assert!(matches!(program[2], Instruction::Add));
    match &program[3] {
        Instruction::IfElse { t_branch, f_branch } => {
            assert!(matches!(t_branch[0], Instruction::Push(3)));
            assert!(matches!(f_branch[0], Instruction::Push(4)));
        }
        _ => panic!("unexpected instruction: {:?}", program[1]),
    }
    assert!(matches!(program[4], Instruction::Mul));
}
