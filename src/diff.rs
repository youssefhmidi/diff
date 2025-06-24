use std::{collections::HashMap, fs::File, io::{self, Read}, usize};

#[derive(Debug)]
pub enum Actions {
    Changed,
    Removed,
    Added
}

#[derive(Debug)]
pub struct Diff {
    action: Actions,
    line: String,
}

#[derive(Debug)]
pub struct Diffs(HashMap<usize, Diff>);

impl Diff {
    pub fn fmt(&self) -> (String, String) {
        let action = match self.action {
            Actions::Changed => "changed",
            Actions::Added   => "added",
            Actions::Removed => "removed",
        };

        (action.to_string(), self.line.to_owned())
    }
}

impl Diffs {
    fn new() -> Self {
        Diffs(HashMap::new())
    }

    pub fn unwrap(self) -> HashMap<usize, Diff> {
        self.0
    }
}

fn get_lines(file: &mut File) -> io::Result<Vec<String>> {
    let mut str = String::new();

    file.read_to_string(&mut str)?;

    Ok(str.split("\n").map(|s| s.to_owned()).collect())
}

/// the diff algorithm
///
/// takes two files directories and returns the diffrences between them
pub fn diff<'a>(file1: &str, file2: &str) -> io::Result<Diffs> {
    let mut f1 = File::open(file1)?;
    let mut f2 = File::open(file2)?;

    let lines1 = get_lines(&mut f1)?;
    let lines2 = get_lines(&mut f2)?;

    let mut diff = Diffs::new();

    let mut slice = lines1.len();


    // new file has les lines than the older version
    if slice > lines2.len() {
        slice = lines2.len();
        for (i, line) in lines1[slice..].into_iter().enumerate() {
            let diffrence = Diff { action : Actions::Removed, line: (*line.clone()).to_string() };
            diff.0.insert(i + 1 + slice, diffrence);
        }
    }

    // new file has new lines
    if slice < lines2.len() {
        for (i, line) in lines2[slice..].into_iter().enumerate() {
            let diffrence = Diff { action : Actions::Added, line: (*line.clone()).to_string() };
            diff.0.insert(i + 1 + slice, diffrence);
        }
    }

    for (i, line) in lines1[..slice].into_iter().enumerate() {
        if lines2[i] != *line {
            let diffrence = Diff { action : Actions::Changed, line: lines2[i].clone() };
            diff.0.insert(i + 1, diffrence);
        }
    }

    Ok(diff)
}
