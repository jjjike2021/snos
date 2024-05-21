use std::path::Path;

use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use starknet_api::deprecated_contract_class::ContractClass as DeprecatedCompiledClass;

use crate::common::contract_fixtures::{get_compiled_class, get_deprecated_compiled_class};

pub fn get_deprecated_feature_contract_class(contract_name: &str) -> DeprecatedCompiledClass {
    let filename = format!("{contract_name}_compiled.json");
    let contract_rel_path =
        Path::new("blockifier_contracts").join("feature_contracts").join("cairo0").join("compiled").join(filename);
    println!("Getting contract at {:?}", contract_rel_path);
    get_deprecated_compiled_class(&contract_rel_path)
}

pub fn get_deprecated_erc20_contract_class() -> DeprecatedCompiledClass {
    let contract_rel_path = Path::new("blockifier_contracts")
        .join("ERC20_without_some_syscalls")
        .join("ERC20")
        .join("erc20_contract_without_some_syscalls_compiled.json");
    get_deprecated_compiled_class(&contract_rel_path)
}

pub fn get_feature_contract_class(contract_name: &str) -> CasmContractClass {
    let filename = format!("{contract_name}.casm.json");
    let contract_rel_path =
        Path::new("blockifier_contracts").join("feature_contracts").join("cairo1").join("compiled").join(filename);
    println!("Getting contract at {:?}", contract_rel_path);
    get_compiled_class(&contract_rel_path)
}