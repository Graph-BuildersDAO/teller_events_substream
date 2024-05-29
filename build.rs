use anyhow::{Ok, Result};
use regex::Regex;
use substreams_ethereum::Abigen;
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    let file_names = [
        "abi/tellerv2_contract.abi.json",
        "abi/marketregistry_contract.abi.json",
        "abi/lendercommitment_contract.abi.json",
        "abi/lendercommitmentstaging_contract.abi.json",
        "abi/collateralmanager_contract.abi.json",
        "abi/lendermanager_contract.abi.json",
        "abi/marketliquidityrewards_contract.abi.json",
        "abi/tellerv0storage_contract.abi.json",
        "abi/erc20.json",
        "abi/erc165.json"
    ];
    let file_output_names = [
        "src/abi/tellerv2_contract.rs",
        "src/abi/marketregistry_contract.rs",
        "src/abi/lendercommitment_contract.rs",
        "src/abi/lendercommitmentstaging_contract.rs",
        "src/abi/collateralmanager_contract.rs",
        "src/abi/lendermanager_contract.rs",
        "src/abi/marketliquidityrewards_contract.rs",
        "src/abi/tellerv0storage_contract.rs",
        "src/abi/erc20.rs",
        "src/abi/erc165.rs"
    ];

    let mut i = 0;
    for f in file_names {
        let contents = fs::read_to_string(f)
            .expect("Should have been able to read the file");

        // sanitize fields and attributes starting with an underscore
        let regex = Regex::new(r#"("\w+"\s?:\s?")_(\w+")"#).unwrap();
        let sanitized_abi_file = regex.replace_all(contents.as_str(), "${1}u_${2}");

        Abigen::from_bytes("Contract", sanitized_abi_file.as_bytes())?
            .generate()?
            .write_to_file(file_output_names[i])?;

        i = i+1;
    }

    Ok(())
}