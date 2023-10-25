mod common;
use std::collections::HashMap;
use std::path::Path;

use cairo_vm::vm::runners::builtin_runner::OUTPUT_BUILTIN_NAME;
use cairo_vm::vm::runners::cairo_pie::{BuiltinAdditionalData, CairoPie, OutputBuiltinAdditionalData, SegmentInfo};
use cairo_vm::vm::runners::cairo_runner::ExecutionResources;
use common::setup_pie;
use rstest::rstest;
use serde_json::json;
use snos::sharp::pie::{decode_base64_to_unzipped, encode_pie, PIE_FILES};

#[rstest]
fn pie_metadata_ok(setup_pie: CairoPie) {
    let pie_metadata = setup_pie.metadata;

    assert_eq!(pie_metadata.ret_pc_segment, SegmentInfo::from((4, 0)));
    assert_eq!(pie_metadata.ret_fp_segment, SegmentInfo::from((3, 0)));
    let expected_builtin_segments = HashMap::from([(String::from("output"), SegmentInfo::from((2, 3)))]);
    assert_eq!(pie_metadata.builtin_segments, expected_builtin_segments);
    assert_eq!(pie_metadata.program_segment, SegmentInfo::from((0, 12)));
    assert_eq!(pie_metadata.execution_segment, SegmentInfo::from((1, 7)));

    let metadata_s = serde_json::to_value(&pie_metadata);
    assert!(metadata_s.is_ok());
}

#[rstest]
fn pie_additional_data_ok(setup_pie: CairoPie) {
    let additional_data = setup_pie.additional_data;

    let expected_additional_data = HashMap::from([(
        OUTPUT_BUILTIN_NAME.to_string(),
        BuiltinAdditionalData::Output(OutputBuiltinAdditionalData {
            pages: HashMap::new(),
            attributes: HashMap::new(),
        }),
    )]);

    assert_eq!(additional_data, expected_additional_data);
    let additional_data_s = serde_json::to_value(&additional_data).unwrap();
    assert_eq!(additional_data_s, json!({"output_builtin": {"pages": {}, "attributes": {}}}));
}

#[rstest]
fn pie_execution_resources_ok(setup_pie: CairoPie) {
    let execution_resources = setup_pie.execution_resources;

    let expected_execution_resources = ExecutionResources {
        n_steps: 8,
        n_memory_holes: 0,
        builtin_instance_counter: HashMap::from([(OUTPUT_BUILTIN_NAME.to_string(), 3)]),
    };
    assert_eq!(execution_resources, expected_execution_resources);

    let execution_resources_s = serde_json::to_value(&execution_resources).unwrap();
    assert_eq!(
        execution_resources_s,
        json!({"n_steps": 8, "n_memory_holes": 0, "builtin_instance_counter": {"output_builtin": 3}})
    );
}

#[rstest]
fn pie_version_ok(setup_pie: CairoPie) {
    let version = setup_pie.version;

    let version_s = serde_json::to_value(version).unwrap();
    assert_eq!(version_s, json!({"cairo_pie": "1.1"}));
}

#[rstest]
fn pie_memory_ok(setup_pie: CairoPie) {
    let expected_memory_bin = include_bytes!("common/data/memory.bin");
    let expected_memory_bin = expected_memory_bin.iter().fold(String::new(), |acc, i| acc + &format!("{i:02x?}"));

    let pie_s = serde_json::to_value(setup_pie).unwrap();
    assert_eq!(expected_memory_bin, pie_s["memory"]);
}

#[rstest]
fn convert_b64_to_raw() {
    decode_base64_to_unzipped(&std::fs::read_to_string("tests/common/data/output_pie.b64").unwrap()).unwrap();

    for file in PIE_FILES {
        assert!(
            Path::new(&format!("build/pie/{file:}.{:}", if file != "memory" { "json" } else { "bin" })).exists(),
            "Missing file {file:}"
        );
    }
}
