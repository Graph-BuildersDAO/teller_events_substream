use substreams::Hex;

pub fn convert_address_to_string(address: &Vec<u8>) -> String {
    format!("0x{}", Hex(address).to_string())
}