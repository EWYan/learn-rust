extern crate json;
use std::fs;
// use json::JsonValue;


#[derive(Debug)]
struct ScriptsT {
    source_file: String,//input bin file
    target_file: String,//output vbf file
    vbf_version: String,//vbf format version 
    sw_type: String, //software type,
    sw_part_nmu: String,//software partnumber
    ecu_addr: String,//ecu address
    sw_version: String,//software version
    create_vbt: bool,//if create verification block table
    vbt_addr: String,//address of vbt
    compressed: bool,//if compress input bin file
    sort: bool,//tbd
    group: bool,//tbd
}
impl ScriptsT {
    fn print_v(&self) {
        println!("current vbb version: {}", self.vbf_version);
    }
}
fn main() {
    let f_json = fs::read_to_string("./vbb_script.json").unwrap();
    let _json_entry = json::parse(&f_json).unwrap();
    //need this type of value index
    let var1 = _json_entry["VBF1"]["VBFVersion"].as_str().unwrap();
    println!("var1: {}", var1);
    let vbb_script:Box<ScriptsT> = Box::new(ScriptsT{
        source_file: String::from(_json_entry["VBF1"]["SourceFile"].as_str().unwrap()),
        target_file: String::from(_json_entry["VBF1"]["TargetFile"].as_str().unwrap()),
        vbf_version: String::from(_json_entry["VBF1"]["VBFVersion"].as_str().unwrap()),
        sw_type: String::from(_json_entry["VBF1"]["SwType"].as_str().unwrap()),
        sw_part_nmu: String::from(_json_entry["VBF1"]["SwPartNum"].as_str().unwrap()),
        ecu_addr: String::from(_json_entry["VBF1"]["ECUaddr"].as_str().unwrap()),
        sw_version: String::from(_json_entry["VBF1"]["SwVersion"].as_str().unwrap()),
        create_vbt: _json_entry["VBF1"]["CreateVerificationBlock"].as_bool().unwrap(),
        vbt_addr: String::from(_json_entry["VBF1"]["VerificationBlockStartAddr"].as_str().unwrap()),
        compressed: _json_entry["VBF1"]["Compressed"].as_bool().unwrap(),
        sort: _json_entry["VBF1"]["Sort"].as_bool().unwrap(),
        group: _json_entry["VBF1"]["Group"].as_bool().unwrap(),
    });
    println!("vbb: {:#?}", vbb_script);
    vbb_script.print_v();
}
