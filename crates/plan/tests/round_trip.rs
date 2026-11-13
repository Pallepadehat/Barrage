// SPDX-License-Identifier: MIT

//! Round-trips a plan that uses every field and variant, in both formats.

// Integration-test helpers fall outside clippy.toml's test exemptions.
#![allow(clippy::unwrap_used, clippy::panic)]

use barrage_plan::{
    Assertion, BodyType, CsvDataSet, Extractor, Format, HeaderCheck, HttpDefaults, HttpRequest,
    JsonPathCheck, KeyValue, Method, Pause, Plan, Step, TextCheck, ThreadGroup, Variable,
};

fn kv(key: &str, value: &str, enabled: bool) -> KeyValue {
    KeyValue {
        key: key.into(),
        value: value.into(),
        enabled,
    }
}

fn full_plan() -> Plan {
    let request = HttpRequest {
        name: "Create order".into(),
        enabled: false,
        method: Method::Put,
        url: "https://api.example.com/orders?x=1".into(),
        query: vec![kv("page", "1", true), kv("debug", "true", false)],
        headers: vec![kv("Authorization", "Bearer ${token}", true)],
        body_type: BodyType::Text,
        // Multi-line, leading/trailing and doubled spaces, quotes, a long line.
        body: Some(format!(
            "  line one\n\nline  \"three\" 'q' #x: y\n{}\n",
            "word  ".repeat(40)
        )),
        timeout_ms: Some(5_000),
        assertions: vec![
            Assertion::Status {
                expected: "200-299,304".into(),
            },
            Assertion::Body {
                check: TextCheck::NotContains,
                text: "error".into(),
            },
            Assertion::JsonPath {
                expr: "$.items[0].id".into(),
                check: JsonPathCheck::Equals,
                value: Some("42".into()),
            },
            Assertion::JsonPath {
                expr: "$.missing".into(),
                check: JsonPathCheck::NotExists,
                value: None,
            },
            Assertion::ResponseTime { max_ms: 300 },
            Assertion::Header {
                name: "Content-Type".into(),
                check: HeaderCheck::Contains,
                value: Some("json".into()),
            },
        ],
        extractors: vec![
            Extractor::JsonPath {
                var: "id".into(),
                expr: "$.id".into(),
                default: Some("NOT_FOUND".into()),
            },
            Extractor::Regex {
                var: "csrf".into(),
                expr: r#"name="csrf" value="([^"]+)""#.into(),
                group: Some(1),
                default: None,
            },
            Extractor::Header {
                var: "loc".into(),
                name: "Location".into(),
                default: None,
            },
        ],
    };
    Plan {
        name: "Everything: a plan".into(),
        variables: vec![Variable {
            key: "n".into(),
            value: "007".into(),
        }],
        defaults: HttpDefaults {
            base_url: Some("http://localhost:8080".into()),
            headers: vec![kv("Accept", "*/*", true)],
            timeout_ms: Some(10_000),
            follow_redirects: Some(false),
            cookies: Some(false),
        },
        csv_data: vec![CsvDataSet {
            path: "data/users.csv".into(),
            variables: vec![],
            delimiter: Some(';'),
            recycle: false,
        }],
        thread_groups: vec![ThreadGroup {
            name: "Paced".into(),
            enabled: false,
            users: 10,
            ramp_up_secs: 0,
            duration_secs: None,
            iterations: Some(100),
            think_time_ms: 0,
            think_jitter_ms: 0,
            target_rps: Some(12.5),
            steps: vec![
                Step::Http(request),
                Step::Pause(Pause {
                    name: "Wait".into(),
                    enabled: true,
                    duration_ms: 250,
                }),
            ],
        }],
    }
}

#[test]
fn every_field_round_trips_through_yaml() {
    let plan = full_plan();
    let text = Format::Yaml.to_string(&plan).unwrap();
    let back = Format::Yaml.parse(&text).unwrap();
    assert_eq!(back, plan, "YAML was:\n{text}");
    assert_eq!(Format::Yaml.to_string(&back).unwrap(), text);
}

#[test]
fn every_field_round_trips_through_json() {
    let plan = full_plan();
    let text = Format::Json.to_string(&plan).unwrap();
    assert_eq!(Format::Json.parse(&text).unwrap(), plan);
}

#[test]
fn defaults_are_omitted_and_restored() {
    let text = "name: Tiny\n";
    let plan = Format::Yaml.parse(text).unwrap();
    assert!(plan.defaults.is_empty() && plan.thread_groups.is_empty());
    assert_eq!(Format::Yaml.to_string(&plan).unwrap(), text);
    assert_eq!(
        Format::Json.to_string(&plan).unwrap(),
        "{\n  \"name\": \"Tiny\"\n}\n"
    );
}
