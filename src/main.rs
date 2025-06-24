/*
 * diff v0.1.0 Beta
 */
mod diff;

use std::{collections::HashMap, env};

const VERSION: &str = "v0.1.0 Beta";

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("you're running diff version {0}.\n\nusage: diff <old_file> <new_file> [--json]\nthe --json flag is optional", VERSION),
        3.. => {
            let res_diffs = diff::diff(args[1].as_str(), args[2].as_str());

            let diffs = match res_diffs {
                Ok(diffs) => diffs.unwrap(),
                Err(e) => return eprintln!("{}", e)
            };

            if let Some(flag) = args.get(3) {
                if flag == "--json" {
                    let mut diff_arrays = Vec::<HashMap<String, String>>::new();

                    for (index, dif) in diffs {
                        let mut table = HashMap::<String, String>::new();

                        table.insert("line".to_string(), format!("{index}").to_string());
                        let (action, text) = dif.fmt();

                        table.insert("action".into(), action);
                        table.insert("text".into(), text);

                        diff_arrays.push(table);
                    }

                    println!("{:?}", diff_arrays);
                    return
                }
            }

            let mut fmt_diffs: Vec<String> = Vec::<String>::new();

            for (line, dif) in diffs {
                let (action,mut text) = dif.fmt();

                if text.ends_with("\r") {
                    text.pop();
                }

                fmt_diffs.push(format!("{text} ({action}) at {line}"));
            }

            for output in fmt_diffs {
                println!("{output}")
            }
        }
        _ => (),
    }
}
