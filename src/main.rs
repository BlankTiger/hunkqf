use std::process::{Command, Stdio};

fn main() {
    let cmd_output = Command::new("git")
        .arg("diff")
        .stdout(Stdio::piped())
        .output()
        .expect("there to be outpuut")
        .stdout;
    let diff = std::str::from_utf8(&cmd_output).unwrap();
    let locs = parse_locs(diff);
    for loc in locs {
        println!("{loc}");
    }
}

fn parse_locs(diff: &str) -> Vec<String> {
    diff.split("+++ b/")
        .enumerate()
        .filter_map(|(idx, l)| if idx % 2 != 0 { Some(l) } else { None })
        .map(|loc| {
            let (filename, rest) = loc.split_once('\n').unwrap();
            let (loc_line, code) = rest.split_once('\n').unwrap();
            let (_, location_dirty) = loc_line.split_once('+').unwrap();
            let location = location_dirty.split(',').next().unwrap();
            let code_line = code.split('\n').next().unwrap();
            let code_line = if code_line.starts_with('+') {
                code_line.strip_prefix('+').unwrap()
            } else if code_line.starts_with('-') {
                code_line.strip_prefix('-').unwrap()
            } else {
                code_line
            };
            format!("{filename}:{location}:0: {code_line}")
        })
        .collect()
}
