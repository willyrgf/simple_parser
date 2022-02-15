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

// const CTRL_TOKENS: &'static [&'static str] = &["begin", "end"];

// fn parse_inst_branches(pos: usize, tokens: &Vec<&str>, branch: &mut Vec<Instruction>) -> usize {
//     let (inst, next_pos) = parse_inst(pos, tokens);
//     match inst {
//         Some(v) => {
//             branch.push(v);
//         }
//         None => return next_pos,
//     }

//     parse_inst_branches(next_pos, tokens, branch)
// }

fn parse_inst(mut pos: usize, tokens: &Vec<&str>) -> (Option<Instruction>, usize) {
    if tokens.len() - 1 < pos {
        return (None, pos);
    }
    println!("parse_inst(): pos: {} tokens[pos]: {}", pos, tokens[pos]);

    let inst = tokens[pos].split(".").collect::<Vec<_>>();
    let instruction = match inst[0] {
        "begin" => None,
        "add" => Some(Instruction::Add),
        "push" => {
            let v = inst[1].parse::<u64>().unwrap();
            Some(Instruction::Push(v))
        }
        "mul" => Some(Instruction::Mul),
        "if" | "else" => {
            let mut t_branch: Vec<Instruction> = vec![];
            let mut f_branch: Vec<Instruction> = vec![];

            loop {
                // println!(
                //     "parse_inst(): loop: pos: {}; tokens[pos]: {}",
                //     pos, tokens[pos]
                // );
                let (instruction, next_pos) = parse_inst(pos + 1, tokens);
                pos = next_pos;

                println!(
                    "parse_inst(): after loop: pos: {}; tokens[pos]: {}; instruction: {:?}; t_branch: {:?}; f_branch: {:?}",
                    pos, tokens[pos], instruction, t_branch, f_branch
                );

                match instruction {
                    Some(v) => match inst[0] {
                        // println!()
                        "if" => {
                            t_branch.push(v);
                            println!("parse_inst(): insert in t_branch: {:?}", t_branch);
                        }
                        "else" => {
                            f_branch.push(v);
                            println!("parse_inst(): insert in f_branch: {:?}", f_branch);
                        }
                        _ => {
                            println!(
                                "parse_inst(): break inst[0]: pos: {}; tokens[pos]: {}",
                                pos, tokens[pos]
                            );
                            break;
                        }
                    },
                    None => {
                        println!(
                            "parse_inst(): break None: pos: {}; tokens[pos]: {}",
                            pos, tokens[pos]
                        );

                        return (instruction, pos);
                        // break;
                    }
                }
            }

            println!(
                "parse_inst(): inside if|else Some(): pos: {}; tokens[pos]: {}; t_branch: {:?}; f_branch: {:?}",
                pos, tokens[pos], t_branch, f_branch
            );
            Some(Instruction::IfElse { t_branch, f_branch })
            // let next_pos: usize;
            // if inst[0] == "if" {
            //     next_pos = parse_inst_branches(pos + 1, tokens, &mut t_branch);
            // } else {
            //     next_pos = parse_inst_branches(pos + 1, tokens, &mut f_branch);
            // }
            // pos = next_pos;
        }
        "endif" => None,
        "end" => None,
        _ => panic!("pos: {}, inst: {:?}, tokens: {:?}", pos, inst, tokens),
    };

    (instruction, pos)
}

pub fn parse(source: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let tokens = source.split_whitespace().collect::<Vec<_>>();

    let mut pos = 0;
    while tokens.len() > pos {
        println!(
            "parse(): while: tokens[pos]:{:?}, pos: {}",
            tokens[pos], pos
        );
        let (inst, next_pos) = parse_inst(pos + 1, &tokens);
        println!(
            "parse(): inst: {:?}, pos: {}, next_pos: {}",
            inst, pos, next_pos
        );
        match inst {
            Some(v) => instructions.push(v),
            None => (),
        }
        pos = next_pos;
        // pos += 1;
    }

    instructions
}

fn main() {}

// TESTS
// =================================================================================

#[test]
fn test_simple() {
    let program = parse("begin push.1 push.2 add end");
    println!("program: {:?}", program);
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
