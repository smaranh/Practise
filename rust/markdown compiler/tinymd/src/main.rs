fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("),");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}

fn parse_markdown_file() {

}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    let mut the_title = String::from("\nWritten by: ");
    the_title.push_str(env!("CARGO_PKG_AUTHORS"));
    the_title.push_str("\nHomepage: ");
    the_title.push_str(env!("CARGO_PKG_HOMEPAGE"));
    the_title.push_str("\nUsage: tinymd <somefile>.md");
    println!("{}", the_title);
}

fn usage() {
    print_long_banner();
}

fn main() {
    usage();
}