use std::path::Path;


pub fn find_files(root_path: &Path, extentions: &[&str], recursive_search: bool) -> Vec<String> {
  let mut files = Vec::new();

  for entry in std::fs::read_dir(root_path).unwrap() {

      let entry = entry.unwrap();
      let path = entry.path();

      if path.is_file() {
          let ext = path.extension();
          if let Some(ext) = ext {
              if extentions.contains(&ext.to_str().unwrap()) {
                  files.push(path.to_str().unwrap().to_string());
              }
          }
      } else if path.is_dir() && recursive_search {
          files.append(&mut find_files(&path, extentions, recursive_search));
      }
  }

  files
}