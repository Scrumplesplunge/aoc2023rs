use std::io;
use std::io::Read;

const MAX_OPS: usize = 2048;
const MAX_WORKFLOWS: usize = 600;

#[derive(Copy, Clone)]
enum Category { X, M, A, S }
#[derive(Copy, Clone)]
enum Action { Accept(), Reject(), Delegate(u16) }
#[derive(Copy, Clone)]
enum Op { IfLess(Category, u16, Action), IfMore(Category, u16, Action), Unconditionally(Action) }
type WorkflowId = u16;

fn read_workflow_name(input: &mut &[u8]) -> WorkflowId {
    let mut index = 0;
    let mut count = 0;
    while let c @ b'a'..=b'z' = input[count] {
        count += 1;
        index = 26 * index + (c - b'a' + 1) as WorkflowId;
    }
    if count == 0 { panic!("expected workflow name") }
    *input = &input[count..];
    return index;
}

fn read_int(input: &mut &[u8]) -> u16 {
    let mut result = 0;
    let mut count = 0;
    // No need to check for the end of the string (no valid input ends with an int).
    while let c @ (b'0'..=b'9') = input[count] {
        result = 10 * result + (c - b'0') as u16;
        count += 1;
    }
    if count == 0 { panic!("expected int") }
    *input = &input[count..];
    return result;
}

fn read_action(input: &mut &[u8]) -> Action {
    match input {
        [b'A', ..] => { *input = &input[1..]; Action::Accept() },
        [b'R', ..] => { *input = &input[1..]; Action::Reject() },
        _ => { Action::Delegate(read_workflow_name(input)) },
    }
}

fn read_workflow(ops: &mut [Op], num_ops: &mut usize, input: &mut &[u8]) -> (WorkflowId, u16) {
    let id = read_workflow_name(input);
    let start = *num_ops;
    if input[0] != b'{' { panic!("syntax") }
    *input = &input[1..];
    // Parse the list of operations for the workflow.
    loop {
        match input {
            // Match a conditional step.
            [c, op @ (b'<' | b'>'), tail @ ..] => {
                let category = match c {
                    b'x' => Category::X,
                    b'm' => Category::M,
                    b'a' => Category::A,
                    b's' => Category::S,
                    _ => panic!("bad variable"),
                };
                *input = tail;
                let threshold = read_int(input);
                if input[0] != b':' { panic!("bad workflow") }
                *input = &input[1..];
                let action = read_action(input);
                ops[*num_ops] = match op {
                    b'<' => Op::IfLess(category, threshold, action),
                    b'>' => Op::IfMore(category, threshold, action),
                    _ => panic!("bad op"),
                };
                *num_ops += 1;
            },
            // Match an unconditional step.
            _ => {
                ops[*num_ops] = Op::Unconditionally(read_action(input));
                *num_ops += 1;
            }
        }
        match input[0] {
            b',' => {},
            b'}' => { break },
            _ => panic!("bad line"),
        }
        *input = &input[1..];
    }
    *input = &input[1..];
    return (id, start as u16);
}

fn read_workflows<'a>(ops: &'a mut [Op], input: &mut &[u8]) -> (&'a [Op], usize) {
    let mut num_ops = 0;

    // `workflows[i]` is a pair `(id, offset)` where `id` is derived from the workflow name and
    // `offset` is the index of the first operation of the workflow in `ops`.
    let mut workflows = [(0, 0); MAX_WORKFLOWS];
    let mut num_workflows = 0;

    while input[0] != b'\n' {
        workflows[num_workflows] = read_workflow(ops, &mut num_ops, input);
        num_workflows += 1;
        if input[0] != b'\n' { panic!("trailing bytes") }
        *input = &input[1..];
    }
    *input = &input[1..];

    // Rewrite all `Delegate(id)` entries to `Delegate(offset)` entries.
    for op in &mut ops[0..num_ops] {
        let action = match op {
            Op::IfLess(_, _, a) => a,
            Op::IfMore(_, _, a) => a,
            Op::Unconditionally(a) => a,
        };
        if let Action::Delegate(x) = action {
            *x = workflows[0..num_workflows].iter().find(|(id, _)| *id == *x).unwrap().1;
        }
    }

    // Identify the starting position.
    let start_id: WorkflowId = read_workflow_name(&mut "in.".as_bytes());
    let start = workflows[0..num_workflows].iter().find(|(id, _)| *id == start_id).unwrap().1;
    return (&ops[0..num_ops], start as usize);
}

fn parse_part(mut text: &[u8]) -> [u16; 4] {
    text = text.strip_prefix(b"{x=").unwrap();
    let x = read_int(&mut text);
    text = text.strip_prefix(b",m=").unwrap();
    let m = read_int(&mut text);
    text = text.strip_prefix(b",a=").unwrap();
    let a = read_int(&mut text);
    text = text.strip_prefix(b",s=").unwrap();
    let s = read_int(&mut text);
    if text != b"}" { panic!("bad part") }
    return [x, m, a, s];
}

fn run(ops: &[Op], start: usize, part: [u16; 4]) -> bool {
    let mut i = start;
    loop {
        let (should_act, action) = match ops[i] {
            Op::IfLess(c, x, action) => (part[c as usize] < x, action),
            Op::IfMore(c, x, action) => (part[c as usize] > x, action),
            Op::Unconditionally(action) => (true, action),
        };
        if should_act {
            match action {
                Action::Accept() => { return true },
                Action::Reject() => { return false },
                Action::Delegate(x) => { i = x as usize },
            }
        } else {
            i += 1;
        }
    }
}

fn count(part: [(u16, u16); 4]) -> u64 {
    let mut total = 1;
    for (a, b) in part {
        total *= (b - a) as u64 + 1;
    }
    return total;
}

fn act(ops: &[Op], part: [(u16, u16); 4], action: Action) -> u64 {
    match action {
        Action::Accept() => count(part),
        Action::Reject() => 0,
        Action::Delegate(j) => eval(ops, j as usize, part),
    }
}

fn eval(ops: &[Op], mut i: usize, mut part: [(u16, u16); 4]) -> u64 {
    let mut accepted = 0;
    loop {
        match ops[i] {
            Op::IfLess(c, x, action) => {
                let (a, b) = part[c as usize];
                if a < x {
                    let mut when_true = part;
                    when_true[c as usize].1 = b.min(x - 1);
                    accepted += act(ops, when_true, action);
                    part[c as usize].0 = x.max(a);
                }
            }
            Op::IfMore(c, x, action) => {
                let (a, b) = part[c as usize];
                if x < b {
                    let mut when_true = part;
                    when_true[c as usize].0 = a.max(x + 1);
                    accepted += act(ops, when_true, action);
                    part[c as usize].1 = x.min(b);
                }
            }
            Op::Unconditionally(action) => {
                return accepted + act(ops, part, action);
            }
        }
        i += 1;
    }
}

fn main() {
    // Read the input into a buffer.
    let mut buffer = [0; 20 * 1024];
    let len = io::stdin().read(&mut buffer).unwrap();
    if len == 0 || buffer[len - 1] != b'\n' { panic!("bad input") }
    let mut input = &buffer[0 .. len - 1];

    // Compile the workflows.
    let mut op_buffer = [Op::Unconditionally(Action::Accept()); MAX_OPS];
    let (ops, start) = read_workflows(&mut op_buffer, &mut input);

    // Process the parts for part 1.
    let mut part1: u32 = 0;
    for part_text in input.split(|b| *b == b'\n') {
        let part = parse_part(part_text);
        if run(ops, start, part) {
            part1 += part[0] as u32 + part[1] as u32 + part[2] as u32 + part[3] as u32;
        }
    }

    // Calculate the hypothetical part count for part 2.
    let part2 = eval(ops, start, [(1, 4000); 4]);

    print!("{}\n{}\n", part1, part2);
}
