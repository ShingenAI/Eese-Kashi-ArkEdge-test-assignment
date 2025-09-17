use std::collections::{BTreeMap, HashSet};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Component {
    #[serde(rename = "type")]
    comp_type: String,
    status: String,
    failure_code: Option<String>,
}

#[derive(Debug, Default)]
struct TypeStats {
    passed: u32,
    failed: u32,
    failures: HashSet<String>,
}

/// Analyze component failures and summarize per component type.
pub fn analyze_component_failures(input_json: &str) -> Vec<String> {
    let components: Vec<Component> = serde_json::from_str(input_json).expect("Invalid JSON input");
    let mut map: BTreeMap<String, TypeStats> = BTreeMap::new();

    for comp in components {
        let stats = map.entry(comp.comp_type.clone()).or_default();

        match comp.status.as_str() {
            "PASSED" => stats.passed += 1,
            "FAILED" => {
                stats.failed += 1;
                if let Some(code) = comp.failure_code {
                    stats.failures.insert(code);
                }
            }
            _ => {}
        }
    }

    let mut result = Vec::new();

    for (comp_type, stats) in map {
        let mut failures: Vec<String> = stats.failures.into_iter().collect();
        failures.sort();
        result.push(format!(
            "{}: PASSED={}, FAILED={}, failures=[{}]",
            comp_type,
            stats.passed,
            stats.failed,
            failures.join(", ")
        ));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_component_failures() {
        let input = r#"
        [
          {"id":"C-001","type":"OBC","status":"FAILED","failure_code":"E101"},
          {"id":"C-002","type":"OBC","status":"PASSED"},
          {"id":"C-003","type":"Antenna","status":"FAILED","failure_code":"E301"},
          {"id":"C-004","type":"Power","status":"PASSED"}
        ]
        "#;

        let output = analyze_component_failures(input);

        let expected = vec![
            "Antenna: PASSED=0, FAILED=1, failures=[E301]",
            "OBC: PASSED=1, FAILED=1, failures=[E101]",
            "Power: PASSED=1, FAILED=0, failures=[]"
        ];

        assert_eq!(output, expected);
    }
}        
/*
create a map: type_stats: Map<String, { passed: u32, failed: u32, failures: Set<String> }>

for each component:
    get the type
    if not in map, insert default entry
    if status is PASSED:
        increment passed
    else if status is FAILED:
        increment failed
        if failure_code exists:
            insert into failure set

for each type in sorted order:
    print summary with sorted unique failures
*/