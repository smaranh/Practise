fn usage() {
    let version: u16 = get_version();
    println!("tinymd is a markdown complier written by Sam");
    println!("version number is {}", version);
}

fn main() {
    usage();
}

fn get_version() -> u16 {
    1000
}