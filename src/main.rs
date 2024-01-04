use std::process::{Command, Stdio};

fn main() {
    let cmd_output = Command::new("git")
        .arg("diff")
        .arg("-U0")
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
        .skip(1)
        .flat_map(|(_, loc)| {
            let (filename, all_code) = loc.split_once('\n').unwrap();
            all_code
                .split("@@ -")
                .skip(1)
                .filter(|hunk| hunk.chars().next().is_some_and(|x| x.is_numeric()))
                .map(|hunk| {
                    let (mut loc_line, code) = hunk.split_once('\n').unwrap();
                    loc_line = if loc_line.contains('\n') {
                        loc_line.split_once('\n').unwrap().0
                    } else {
                        loc_line
                    };
                    let location_dirty = loc_line
                        .split_once('+')
                        .unwrap()
                        .1
                        .split_once(' ')
                        .unwrap()
                        .0;
                    let location = if location_dirty.contains(',') {
                        location_dirty.split(',').next().unwrap()
                    } else {
                        location_dirty
                    };
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
                .collect::<Vec<_>>()
        })
        .collect()
}
