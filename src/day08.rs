enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter_map(|ins| {
            let (ins, arg) = ins.split_once(' ')?;
            let arg = arg.parse().ok()?;

            match ins {
                "acc" => Some(Instruction::Acc(arg)),
                "jmp" => Some(Instruction::Jmp(arg)),
                "nop" => Some(Instruction::Nop(arg)),
                _ => None,
            }
        })
        .collect()
}

fn run_program(instructions: &[Instruction]) -> Result<isize, isize> {
    let (mut ins, mut acc) = (0, 0);
    let mut visited = (0..instructions.len()).map(|_| false).collect::<Vec<_>>();

    loop {
        match visited.get_mut(ins) {
            Some(was_visited) => match was_visited {
                false => {
                    *was_visited = true;

                    match instructions[ins] {
                        Instruction::Acc(arg) => acc += arg,
                        Instruction::Jmp(arg) => ins = (ins as isize + arg - 1) as usize,
                        Instruction::Nop(_) => {}
                    };

                    ins += 1;
                }
                true => break Err(acc),
            },
            None => break Ok(acc),
        }
    }
}

pub fn part1(input: &str) -> isize {
    let instructions = parse(input);

    match run_program(&instructions) {
        Err(acc) => acc,
        Ok(_) => unreachable!(),
    }
}

pub fn part2(input: &str) -> isize {
    let mut instructions = parse(input);

    for i in 0..instructions.len() {
        let ins = &mut instructions[i];

        match ins {
            Instruction::Nop(n) => *ins = Instruction::Jmp(*n),
            Instruction::Jmp(n) => *ins = Instruction::Nop(*n),
            _ => continue,
        }

        if let Ok(acc) = run_program(&instructions) {
            return acc;
        } else {
            let ins = &mut instructions[i];
            match ins {
                Instruction::Nop(n) => *ins = Instruction::Jmp(*n),
                Instruction::Jmp(n) => *ins = Instruction::Nop(*n),
                _ => {}
            }
        }
    }

    unreachable!()
}
