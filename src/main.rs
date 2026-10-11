use std::env;
use std::fs::read_dir;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let target_dir = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let list = read_dir(target_dir).expect("ERROR");

    let mut entries: Vec<_> = Vec::new();
    for entry in list {
        let entry = entry.expect("ERROR");
        entries.push(entry);
    }
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let file_name = entry.file_name();
        let metadata = entry.metadata().expect("ERROR");
        let is_dir = if metadata.is_dir() { "d" } else { "-" };
        let mode = metadata.permissions().mode();
        let perm = parse_permissions(mode);
        let name = file_name.to_string_lossy();

        print!("{is_dir}{perm} ");
        print!("{:>8} ", metadata.len());
        println!("{name}");
    }
}

fn parse_permissions(mode: u32) -> String {
    let mut perm = String::new();
    perm.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
    perm.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
    perm.push(if (mode & 0o100) != 0 { 'x' } else { '-' });

    perm.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
    perm.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
    perm.push(if (mode & 0o010) != 0 { 'x' } else { '-' });

    perm.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
    perm.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
    perm.push(if (mode & 0o001) != 0 { 'x' } else { '-' });

    perm
}
