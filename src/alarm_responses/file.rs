#![allow(clippy::module_name_repetitions)]
use crate::config::FileArguments;

pub fn alarm(path_arg: &FileArguments) {
    if let Err(e) = std::process::Command::new(&path_arg.file_path)
        .current_dir(&path_arg.run_directory.clone().unwrap_or_default())
        .spawn()
    {
        log::error!(
            "Failed to launch file {} in {} with error {e}",
            path_arg.file_path.display(),
            path_arg.run_directory.clone().unwrap_or_default().display(),
        );
    }
}
