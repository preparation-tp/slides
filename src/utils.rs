use std::path::{Path, PathBuf};
use std::fs;

pub fn get_slides(slide_dir: &str) -> Vec<String> {
    let mut slides = Vec::new();

    fn visit_dirs(dir: &Path, base: &Path, slides: &mut Vec<String>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, base, slides)?;
                } else if path.is_file() {
                    match path.extension().and_then(|ext| ext.to_str()) {
                        Some("html") => {
                            if let Some(name) = path.strip_prefix(base).ok().and_then(|p| p.to_str()) {
                                let mut slide_path = PathBuf::from(name);
                                slide_path.set_extension(""); // Remove extension
                                if let Some(slide_name) = slide_path.to_str() {
                                    slides.push(slide_name.trim_end_matches('.').to_string());
                                }
                            }
                        },
                        _ => (),
                    }
                }
            }
        }
        Ok(())
    }

    let base_path = Path::new(slide_dir);
    if let Err(e) = visit_dirs(base_path, base_path, &mut slides) {
        eprintln!("Error reading directory {}: {}", slide_dir, e);
    }
    slides
}
