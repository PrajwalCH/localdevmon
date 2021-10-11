use std::io;
use std::path::PathBuf;

pub fn map_route(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut routes = Vec::<PathBuf>::new();

    path.read_dir().and_then(|entries| {
        for entry in entries {
            if let Ok(entry) = entry {
                let sub_path = entry.path();

                if !is_hidden(&sub_path) {
                    routes.push(sub_path.clone());

                    if sub_path.is_dir() {
                        let sub_routes = map_route(&sub_path)?;

                        routes.extend(sub_routes);
                    }
                }
            }
        }

        Ok(routes)
    })
}

fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|base_name| base_name.to_str())
        .map_or_else(|| false, |base_name| base_name.starts_with("."))
}
