#[derive(Debug, Clone)]
pub enum Instruction {
    Push(u64),
    Add,
    Mul,
    IfElse {
        t_branch: Vec<Instruction>,
        f_branch: Vec<Instruction>,
    },
}

fn parse_inst(
    mut pos: usize,
    tokens: &Vec<&str>,
    t_branch: &mut Vec<Instruction>,
    f_branch: &mut Vec<Instruction>,
) -> (Option<Instruction>, usize) {
    if tokens.len() - 1 < pos {
        return (None, pos);
    }

    let inst = tokens[pos].split(".").collect::<Vec<_>>();
    let instruction = match inst[0] {
        "begin" => None,
        "add" => Some(Instruction::Add),
        "push" => {
            let v = inst[1].parse::<u64>().unwrap();
            Some(Instruction::Push(v))
        }
        "mul" => Some(Instruction::Mul),
        "if" => loop {
            let (instruction, next_pos) = parse_inst(pos + 1, tokens, t_branch, f_branch);
            pos = next_pos;

            if tokens[pos] == "endif" {
                return (instruction, pos);
            }

            match instruction {
                Some(v) => t_branch.push(v),
                None => return (None, pos),
            }
        },
        "else" => loop {
            let (instruction, next_pos) = parse_inst(pos + 1, tokens, t_branch, f_branch);
            pos = next_pos;

            if tokens[pos] == "endif" {
                return (instruction, pos);
            }

            match instruction {
                Some(v) => f_branch.push(v),
                None => return (None, pos),
            }
        },
        "endif" => Some(Instruction::IfElse {
            t_branch: t_branch.to_vec(),
            f_branch: f_branch.to_vec(),
        }),
        "end" => None,
        _ => panic!("pos: {}, inst: {:?}, tokens: {:?}", pos, inst, tokens),
    };

    (instruction, pos)
}

pub fn parse(source: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut t_branch: Vec<Instruction> = vec![];
    let mut f_branch: Vec<Instruction> = vec![];
    let tokens = source.split_whitespace().collect::<Vec<_>>();

    let mut pos = 0;
    while tokens.len() > pos {
        let (inst, next_pos) = parse_inst(pos + 1, &tokens, &mut t_branch, &mut f_branch);
        match inst {
            Some(v) => instructions.push(v),
            None => (),
        }
        pos = next_pos;
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
