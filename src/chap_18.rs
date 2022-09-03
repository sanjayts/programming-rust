use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind::Interrupted;
use std::io::{self, BufRead, BufReader, Cursor, Read, Write};
use std::path::Path;
use std::process::Command;
use std::time::UNIX_EPOCH;
use time::Duration;

#[test]
fn test_copy() {
    let mut source = Cursor::new([1, 2, 3_u8]);
    let mut dest: Vec<u8> = Vec::new();
    let result = copy(&mut source, &mut dest);

    assert_eq!(result.ok(), Some(3));
    assert_eq!(dest, vec![1, 2, 3]);
}

pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn copy<R: Read, W: Write>(src: &mut R, dest: &mut W) -> io::Result<u64> {
    let mut buf = [0_u8; DEFAULT_BUF_SIZE];
    let mut written = 0_u64;
    loop {
        let len = match src.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == Interrupted => continue,
            Err(e) => return Err(e),
        };
        dest.write_all(&buf[..len])?;
        written += len as u64;
    }
}

#[test]
fn test_collect_lines() {
    let reader = BufReader::new(File::open("test_data/words.txt").unwrap());

    // This is a very interesting trick to collecting all lines under a single result
    // as opposed to explicitly looping over the lines iterator and collecting it. This
    // is made possible due to the FromIterator trait implemented in result.rs stdlib.
    let lines = reader.lines().collect::<Result<Vec<String>, _>>();

    assert_eq!(
        lines.ok().unwrap(),
        vec![
            "Life before death",
            "Strength before weakness",
            "Journey before destination"
        ]
    );
}

#[test]
fn test_file_ops() {
    let result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("test_data/words.txt");

    assert_eq!(
        result.err().unwrap().to_string(),
        "File exists (os error 17)"
    );
}

#[cfg(not(windows))]
#[test]
fn test_subprocess() {
    let mut child = Command::new("grep")
        .arg("-i")
        .arg("death")
        .arg("test_data/words.txt")
        .output()
        .expect("Was not expecting grep to fail");

    // https://stackoverflow.com/questions/21011330/
    // https://stackoverflow.com/questions/70341205/how-to-save-command-stdout-to-a-file
    let line = String::from_utf8_lossy(&child.stdout);

    assert_eq!(line, "Life before death\n");
}

#[test]
fn test_path_ops() {
    let p = Path::new("src/lib.rs/");

    assert_eq!(p.file_name(), Some(OsStr::new("lib.rs")));

    let components = p.components().map(|c| c.as_os_str()).collect::<Vec<_>>();
    assert_eq!(components, vec![OsStr::new("src"), OsStr::new("lib.rs")]);

    let p = Path::new("test_data/words.txt");
    let metadata = p.metadata().unwrap();

    assert_eq!(metadata.len(), 69);
    assert_eq!(metadata.file_type().is_file(), true);
    assert_eq!(
        metadata
            .created()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap(),
        Duration::new(1660069207, 571948411)
    );

    let p = Path::new(".");
    let mut paths = vec![];
    for result in p.read_dir().unwrap() {
        let entry = result.unwrap();
        let file_name = entry.file_name();
        paths.push(file_name);
    }
    assert_eq!(
        paths,
        vec![
            "Cargo.toml",
            "LICENSE",
            "target",
            "test_data",
            "Cargo.lock",
            "README.md",
            ".gitignore",
            ".git",
            ".idea",
            "src"
        ]
    );
}
