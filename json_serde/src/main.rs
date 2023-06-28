mod processor;

use processor::Processor;

fn main() {
    let mut processor = Processor::new();
    processor.set_name(String::from("i7-1165"));

    let processor_json = serde_json::to_string_pretty(&processor).unwrap();
    println!("Json:");
    println!("{}", processor_json);

    let processor_struct: Processor = serde_json::from_str(&processor_json).unwrap();
    println!("Struct:");
    println!("{:?}", processor_struct);
}
