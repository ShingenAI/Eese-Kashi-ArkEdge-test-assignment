use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Component {
    id: String,
    comp_type: String,
    status: String,
}

fn main() {
    let components = vec![
        Component {
            id: "C-001".to_string(),
            comp_type: "OBC".to_string(),
            status: "PASSED".to_string(),
        },
        Component {
            id: "C-002".to_string(),
            comp_type: "OBC".to_string(),
            status: "FAILED".to_string(),
        },
        Component {
            id: "C-003".to_string(),
            comp_type: "Antenna".to_string(),
            status: "PASSED".to_string(),
        },
        Component {
            id: "C-004".to_string(),
            comp_type: "Power".to_string(),
            status: "PASSED".to_string(),
        },
    ];

    // HashMap<comp_type, (passed_count, failed_count)>
    let mut status_map: HashMap<String, (u32, u32)> = HashMap::new();

    for comp in &components {
        let entry = status_map.entry(comp.comp_type.clone()).or_insert((0, 0));
        match comp.status.as_str() {
            "PASSED" => entry.0 += 1,
            "FAILED" => entry.1 += 1,
            _ => (), // ignore unknown statuses
        }
    }

    // Sorted output
    let mut types: Vec<&String> = status_map.keys().collect();
    types.sort();

    for comp_type in types {
        let (passed, failed) = status_map[comp_type];
        println!("{}: PASSED={}, FAILED={}", comp_type, passed, failed);
    }
}