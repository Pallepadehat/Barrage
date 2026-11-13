// SPDX-License-Identifier: MIT

//! Round-trip tests on the checked-in example plans.

// Integration-test helpers fall outside clippy.toml's test exemptions.
#![allow(clippy::unwrap_used, clippy::panic)]

use std::path::PathBuf;

use barrage_plan::{Format, Plan};

fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name)
}

fn read(name: &str) -> String {
    std::fs::read_to_string(example(name)).unwrap()
}

#[test]
fn yaml_to_model_to_yaml_is_byte_identical() {
    let text = read("checkout.barrage.yaml");
    let plan = Format::Yaml.parse(&text).unwrap();
    assert_eq!(Format::Yaml.to_string(&plan).unwrap(), text);
}

#[test]
fn yaml_to_json_to_yaml_is_lossless() {
    let text = read("checkout.barrage.yaml");
    let plan = Format::Yaml.parse(&text).unwrap();
    let json = Format::Json.to_string(&plan).unwrap();
    let back: Plan = Format::Json.parse(&json).unwrap();
    assert_eq!(back, plan);
    assert_eq!(Format::Yaml.to_string(&back).unwrap(), text);
}

#[test]
fn json_twin_matches_yaml_example() {
    let from_yaml = barrage_plan::load(example("checkout.barrage.yaml")).unwrap();
    let json = read("checkout.barrage.json");
    assert_eq!(Format::Json.to_string(&from_yaml).unwrap(), json);
    assert_eq!(
        barrage_plan::load(example("checkout.barrage.json")).unwrap(),
        from_yaml
    );
}

/// The plan exactly as the PRD prints it, flow style included.
const PRD_PLAN: &str = r#"name: Checkout API
variables:
  - { key: host, value: https://api.example.com }
defaults:
  baseUrl: ${host}
  timeoutMs: 30000
  cookies: true
csvData:
  - { path: users.csv, variables: [email, password], recycle: true }
threadGroups:
  - name: Shoppers
    users: 200
    rampUpSecs: 60
    durationSecs: 300
    thinkTimeMs: 500
    thinkJitterMs: 250
    steps:
      - type: http
        name: Login
        method: POST
        url: /login
        bodyType: json
        body: '{"email":"${email}","password":"${password}"}'
        assertions:
          - { type: status, expected: 2xx }
        extractors:
          - { kind: jsonPath, var: token, expr: $.token }
"#;

#[test]
fn prd_flow_style_plan_loads_to_the_example() {
    let example = barrage_plan::load(example("checkout.barrage.yaml")).unwrap();
    assert_eq!(Format::Yaml.parse(PRD_PLAN).unwrap(), example);
}

#[test]
fn example_matches_the_prd_plan() {
    let plan = barrage_plan::load(example("checkout.barrage.yaml")).unwrap();
    assert_eq!(plan.name, "Checkout API");
    assert_eq!(plan.defaults.base_url.as_deref(), Some("${host}"));
    let group = &plan.thread_groups[0];
    assert_eq!((group.users, group.duration_secs), (200, Some(300)));
    let barrage_plan::Step::Http(login) = &group.steps[0] else {
        panic!("first step should be an HTTP request");
    };
    assert_eq!(login.method, barrage_plan::Method::Post);
    assert_eq!(login.body_type, barrage_plan::BodyType::Json);
}
