// SPDX-License-Identifier: MIT

//! Unknown fields, bad input and file handling.

// Integration-test helpers fall outside clippy.toml's test exemptions.
#![allow(clippy::unwrap_used, clippy::panic)]

use std::path::PathBuf;

use barrage_plan::{Format, PlanError};

fn parse_err(format: Format, text: &str) -> (PlanError, String) {
    let err = format.parse(text).unwrap_err();
    let message = err.to_string();
    (err, message)
}

#[test]
fn unknown_top_level_field_is_rejected_with_line() {
    let (err, message) = parse_err(Format::Yaml, "name: x\nthreads: 3\n");
    assert!(message.contains("threads"), "{message}");
    let PlanError::Parse { location, .. } = err else {
        panic!("expected a parse error, got {err:?}");
    };
    assert_eq!(location.line, Some(2), "{message}");
}

#[test]
fn unknown_nested_fields_are_rejected() {
    let cases = [
        "name: x\nthreadGroups:\n  - name: g\n    users: 1\n    userz: 2\n",
        "name: x\nthreadGroups:\n  - name: g\n    users: 1\n    steps:\n      - type: http\n        name: r\n        method: GET\n        url: /\n        verb: GET\n",
        "name: x\nthreadGroups:\n  - name: g\n    users: 1\n    steps:\n      - type: pause\n        name: p\n        durationMs: 1\n        extra: 1\n",
        "name: x\nthreadGroups:\n  - name: g\n    users: 1\n    steps:\n      - type: http\n        name: r\n        method: GET\n        url: /\n        assertions:\n          - type: status\n            expected: 2xx\n            exact: true\n",
        "name: x\ndefaults:\n  baseURL: http://localhost\n",
    ];
    for text in cases {
        let (_, message) = parse_err(Format::Yaml, text);
        assert!(message.contains("unknown field"), "{text}\n=> {message}");
    }
}

#[test]
fn unknown_field_in_json_is_rejected_with_line() {
    let (err, message) = parse_err(Format::Json, "{\n  \"name\": \"x\",\n  \"nmae\": \"y\"\n}");
    assert!(message.contains("unknown field `nmae`"), "{message}");
    let PlanError::Parse { location, .. } = err else {
        panic!("expected a parse error, got {err:?}");
    };
    assert_eq!(location.line, Some(3));
}

#[test]
fn unknown_step_type_is_rejected() {
    let text =
        "name: x\nthreadGroups:\n  - name: g\n    users: 1\n    steps:\n      - type: grpc\n";
    let (_, message) = parse_err(Format::Yaml, text);
    assert!(message.contains("grpc"), "{message}");
}

#[test]
fn malformed_yaml_is_an_error_not_a_panic() {
    for text in [
        "",
        ":",
        "name: [unclosed",
        "- a\n- b\n",
        "name: x\nusers: \u{0}",
    ] {
        assert!(Format::Yaml.parse(text).is_err(), "{text:?}");
    }
}

#[test]
fn format_comes_from_the_extension() {
    let yaml = Format::from_path("a/plan.barrage.yaml".as_ref()).unwrap();
    assert_eq!(yaml, Format::Yaml);
    assert_eq!(Format::from_path("p.yml".as_ref()).unwrap(), Format::Yaml);
    assert_eq!(
        Format::from_path("p.barrage.JSON".as_ref()).unwrap(),
        Format::Json
    );
    let err = Format::from_path("plan.toml".as_ref()).unwrap_err();
    assert!(matches!(err, PlanError::UnsupportedExtension { .. }));
}

#[test]
fn save_then_load_in_both_formats() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("plan-io");
    std::fs::create_dir_all(&dir).unwrap();
    let plan = Format::Yaml.parse("name: Saved\n").unwrap();
    for file in ["saved.barrage.yaml", "saved.barrage.json"] {
        let path = dir.join(file);
        barrage_plan::save(&path, &plan).unwrap();
        assert_eq!(barrage_plan::load(&path).unwrap(), plan);
    }
}

#[test]
fn load_errors_name_the_file() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("plan-errors");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bad.barrage.yaml");
    std::fs::write(&path, "name: x\nbogus: 1\n").unwrap();
    let message = barrage_plan::load(&path).unwrap_err().to_string();
    assert!(
        message.starts_with(&format!("{}:2:", path.display())),
        "{message}"
    );

    let missing = barrage_plan::load(dir.join("missing.barrage.yaml")).unwrap_err();
    assert!(matches!(missing, PlanError::Io { .. }));
}

#[test]
fn messages_put_the_location_first_once() {
    let (_, yaml) = parse_err(Format::Yaml, "name: x\nthreads: 3\n");
    assert_eq!(
        yaml,
        "<plan>:2:1: unknown field `threads`, expected one of name, variables, defaults, csvData, threadGroups"
    );
    let (_, json) = parse_err(Format::Json, "{\"name\": 3}");
    assert_eq!(
        json,
        "<plan>:1:10: invalid type: integer `3`, expected a string"
    );
}

#[test]
fn bare_numbers_are_accepted_in_text_fields() {
    let text = "name: x\nthreadGroups:\n  - name: g\n    users: 1\n    steps:\n      - type: http\n        name: r\n        method: GET\n        url: /\n        headers:\n          - { key: X-Retry, value: 3 }\n        body: 123\n        assertions:\n          - { type: status, expected: 200 }\n          - { type: jsonPath, expr: $.ok, check: equals, value: true }\n        extractors:\n          - { kind: header, var: count, name: X-Count, default: 0 }\n";
    let plan = Format::Yaml.parse(text).unwrap();
    let saved = Format::Yaml.to_string(&plan).unwrap();
    for quoted in [
        r#"value: "3""#,
        r#"body: "123""#,
        r#"expected: "200""#,
        r#"value: "true""#,
        r#"default: "0""#,
    ] {
        assert!(saved.contains(quoted), "missing {quoted} in\n{saved}");
    }
    assert_eq!(Format::Yaml.parse(&saved).unwrap(), plan);
}

#[test]
fn variables_keep_their_exact_spelling() {
    let plan = Format::Yaml
        .parse("name: x\nvariables:\n  - { key: id, value: 007 }\n")
        .unwrap();
    assert_eq!(plan.variables[0].value, "007");
}

#[test]
fn non_finite_target_rps_is_rejected() {
    for value in [".inf", "-.inf", ".nan"] {
        let text =
            format!("name: x\nthreadGroups:\n  - name: g\n    users: 1\n    targetRps: {value}\n");
        let (_, message) = parse_err(Format::Yaml, &text);
        assert!(message.contains("finite"), "{value}: {message}");
    }
}

#[test]
fn save_replaces_an_existing_file_and_leaves_no_temp_files() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("plan-atomic");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("p.barrage.yaml");
    std::fs::write(&path, "old contents that are longer than the new ones\n").unwrap();
    barrage_plan::save(&path, &Format::Yaml.parse("name: New\n").unwrap()).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "name: New\n");
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
}
