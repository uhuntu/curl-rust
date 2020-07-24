use std::str;

fn main() {
    let mut handle = curl::easy::Easy::new();
    handle.url("https://globalcdn.nuget.org/packages/microsoft.ai.machinelearning.1.3.0.nupkg").unwrap();
    // handle.nobody(true).unwrap();
    handle.header_function(|header| {
        print!("header: {}", str::from_utf8(header).unwrap());
        true
    }).unwrap();
    handle.perform().unwrap();
}