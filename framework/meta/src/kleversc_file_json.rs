use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::Path};

use crate::abi_json::{BuildInfoAbiJson, ContractAbiJson};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KleverscFileJson {
    pub build_info: BuildInfoAbiJson,
    pub abi: ContractAbiJson,
    pub size: usize,
    pub code: String,
}

pub fn serialize_kleversc_file_json(kleversc_file_json: &KleverscFileJson) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    kleversc_file_json.serialize(&mut ser).unwrap();
    let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
    serialized.push('\n');
    serialized
}

pub fn save_kleversc_file_json(kleversc_file_json: &KleverscFileJson, path: impl AsRef<Path>) {
    let kleversc_file_string = serialize_kleversc_file_json(kleversc_file_json);
    let mut kleversc_file = File::create(path).unwrap();
    write!(kleversc_file, "{kleversc_file_string}").unwrap();
}
