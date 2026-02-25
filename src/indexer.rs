use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct AppInfo {
    pub name: String,
    pub path: PathBuf,
}

pub fn get_installed_apps() -> Vec<AppInfo> {
    let mut apps = Vec::new();
    let mut search_paths = Vec::new();

    // user start menu
    if let Ok(appdata) = env::var("APPDATA") {
        search_paths.push(PathBuf::from(appdata).join("Microsoft\\Windows\\Start Menu\\Programs"));
    }

    // system start menu
    if let Ok(programdata) = env::var("PROGRAMDATA") {
        search_paths.push(PathBuf::from(programdata).join("Microsoft\\Windows\\Start Menu\\Programs"));
    }

    // scan directoriies
    for base_path in search_paths {
        if !base_path.exists() {
            continue;
        }

        for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    // ignore everything except lnk
                    if ext.to_ascii_lowercase() == "lnk" {
                        if let Some(file_stem) = path.file_stem() {
                            let name = file_stem.to_string_lossy().into_owned();
                            
                            // filter things that have uninstall out 
                            if !name.to_lowercase().contains("uninstall") {
                                apps.push(AppInfo {
                                    name,
                                    path: path.to_path_buf(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // sort alphabetically and remove dupes
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps.dedup_by(|a, b| a.name == b.name);

    apps
}