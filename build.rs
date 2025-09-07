// build.rs
// Copies resources to target/debug/resources during build
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .join("resources");
    let src_dir = Path::new("resources");
    if src_dir.exists() {
        let _ = fs::create_dir_all(&target_dir);
        for entry in fs::read_dir(src_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap();
                let dest = target_dir.join(filename);
                let _ = fs::copy(&path, &dest);
            }
        }
    }
}
