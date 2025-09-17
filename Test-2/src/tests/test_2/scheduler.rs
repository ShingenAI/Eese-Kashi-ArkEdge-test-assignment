use crate::tests::shared::command::Command;

pub fn run() {
    let inputs = vec![
        (0, 5),
        (4, 4),
        (10, 3),
    ];

    let mut commands: Vec<Command> = inputs.iter().enumerate()
        .map(|(i, &(start, dur))| Command::new(i + 1, start, dur))
        .collect();

    commands.sort_by_key(|cmd| cmd.start);

    let mut executed: Vec<usize> = Vec::new();
    let mut current_end = 0;

    for cmd in commands {
        if cmd.start >= current_end {
            executed.push(cmd.index);
            current_end = cmd.end;
        }
    }

    println!("Executed Commands: {:?}", executed);
}
