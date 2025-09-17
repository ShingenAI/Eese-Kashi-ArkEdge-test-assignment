/// Represents a command from the ground station.
/*
 Candidate for promotion to its own module if this system grows:
 - Shared by other entities (scheduler, simulator, timeline)
 - Gains domain methods: .overlaps_with(), .preempts(), .duration()
 Current inline form keeps test context simple and focused.
*/
#[derive(Debug, Clone)]
pub struct Command {
    pub id: u32,
    pub start: u32,
    pub end: u32,
    pub priority: u32,
}

/// Schedules commands based on preemption and priority rules.
pub fn schedule_commands(input: Vec<(u32, u32, u32)>) -> Vec<u32> {
    let mut commands: Vec<Command> = input
        .into_iter()
        .enumerate()
        .map(|(i, (start, duration, priority))| Command {
            id: (i + 1) as u32,
            start,
            end: start + duration,
            priority,
        })
        .collect();

    // Sort by start time first
    commands.sort_by_key(|cmd| cmd.start);

    let mut executed: Vec<Command> = Vec::new();

    for cmd in commands {
        let mut overlap = false;
        let mut replaced = vec![];

        for (i, existing) in executed.iter().enumerate() {
            if cmd.start < existing.end && cmd.end > existing.start {
                overlap = true;
                if cmd.priority > existing.priority {
                    replaced.push(i);
                } else {
                    overlap = true;
                    break;
                }
            }
        }

        if replaced.len() > 0 {
            for &i in replaced.iter().rev() {
                executed.remove(i);
            }
            executed.push(cmd);
        } else if !overlap {
            executed.push(cmd);
        }
    }

    executed.sort_by_key(|c| c.start);
    executed.iter().map(|c| c.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_commands() {
        let input = vec![(0, 5, 1), (3, 4, 2), (10, 2, 1)];
        let result = schedule_commands(input);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_priority_same_no_preempt() {
        let input = vec![(0, 5, 1), (3, 4, 1)];
        let result = schedule_commands(input);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_higher_priority_full_preempt() {
        let input = vec![(0, 10, 1), (1, 5, 3)];
        let result = schedule_commands(input);
        assert_eq!(result, vec![2]);
    }
}

/*
define Command struct with id, start, end, priority

sort commands by start time, then by priority (desc), then by ID (asc)

initialize timeline: current_time = 0
initialize schedule = empty list

for each command:
    if it does not overlap any previously scheduled segment:
        add it to schedule
    else if it overlaps:
        compare with scheduled command in that time window
        if this command has higher priority:
            remove overlapping part from existing
            insert this command
        else:
            skip this one

return list of command IDs that ended up in final schedule (sorted by start time)
*/