#[allow(dead_code)]
pub fn log(msg: &str) {
    const LOG_PATH: &'static str = "./peep.log";
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut w = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH).unwrap();
    writeln!(&mut w, "{}", msg);
}

