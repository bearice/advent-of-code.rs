use advent_of_code::common::{Program, ProgramOutput};

fn main() {
    let code = Program::from_file("input21.txt");
    let inputs = ["NOT C T", "AND D T", "NOT A J", "OR T J", "WALK"];
    println!("{:?}", run(&code, &inputs));
    let inputs = [
        "NOT C J", "AND D J", "AND H J", "NOT B T", "AND D T", "OR T J", "NOT A T", "OR T J", "RUN",
    ];
    println!("{:?}", run(&code, &inputs));
}

fn run(code: &Program, inputs: &[&str]) -> Option<i64> {
    let mut code = code.clone();
    let inputs = inputs.join("\n") + "\n";
    let mut inputs = inputs.as_bytes().iter().map(|x| *x as i64);
    let mut output = None;
    let mut input = None;
    loop {
        match code.run_program(input) {
            ProgramOutput::Output(x) => {
                if x > 128 {
                    // println!("{}", x);
                    output = Some(x);
                    // } else {
                    //     print!("{}", (x as u8) as char);
                }
                input = None;
            }
            ProgramOutput::NeedInput => {
                input = inputs.next();
            }
            ProgramOutput::Halt => break,
        }
    }
    output
}
