use crate::folder_structure::{
    DirectoryType::{Contract, Lib},
    RelevantDirectory,
};
use crate::version::FrameworkVersion;
use colored::Colorize;
use std::path::Path;

pub fn print_upgrading(dir: &RelevantDirectory) {
    if let Some((from_version, to_version)) = &dir.upgrade_in_progress {
        println!(
            "\n{}",
            format!(
                "Upgrading from {} to {} in {}\n",
                from_version,
                to_version,
                dir.path.display(),
            )
            .purple()
        );
    }
}

pub fn print_post_processing(dir: &RelevantDirectory) {
    if let Some((from_version, to_version)) = &dir.upgrade_in_progress {
        println!(
            "\n{}",
            format!(
                "Post-processing after upgrade from {} to {} in {}\n",
                from_version,
                to_version,
                dir.path.display(),
            )
            .purple()
        );
    }
}

pub fn print_upgrading_all(from_version: &FrameworkVersion, to_version: &FrameworkVersion) {
    println!(
        "\n{}",
        format!(
            "Upgrading from {} to {} across crates ...",
            from_version, to_version
        )
        .purple()
    );
}

pub fn print_version_change(
    path: &Path,
    deps_name: &str,
    framework_crate_name: &str,
    from_version: &str,
    to_version: &str,
) {
    println!(
        "{}/{}/{}: {} -> {}",
        path.display(),
        deps_name,
        framework_crate_name.underline(),
        format!("\"{from_version}\"").red().strikethrough(),
        format!("\"{to_version}\"").green()
    )
}

pub fn print_rename(old_path: &Path, new_path: &Path) {
    println!(
        "Renaming {} -> {}",
        old_path.display().to_string().red().strikethrough(),
        new_path.display().to_string().green(),
    );
}

pub fn print_tree_dir_metadata(dir: &RelevantDirectory, last_version: &FrameworkVersion) {
    match dir.dir_type {
        Contract => print!(" {}", "[contract]".blue()),
        Lib => print!(" {}", "[lib]".magenta()),
    }

    let version_string = format!("[{}]", dir.version.semver);
    if dir.version.semver == *last_version {
        print!(" {}", version_string.green());
    } else {
        print!(" {}", version_string.red());
    };
}

#[allow(dead_code)]
pub fn print_cargo_dep_remove(path: &Path, dep_name: &str) {
    println!(
        "{}/dependencies/{}",
        path.display(),
        dep_name.red().strikethrough(),
    );
}

#[allow(dead_code)]
pub fn print_cargo_dep_add(path: &Path, dep_name: &str) {
    println!(
        "{}/dependencies/{}",
        path.display(),
        dep_name.red().strikethrough(),
    );
}

pub fn print_cargo_check(dir: &RelevantDirectory) {
    println!(
        "\n{}",
        format!(
            "Running cargo check after upgrading to version {} in {}\n",
            dir.version.semver,
            dir.path.display(),
        )
        .purple()
    );
}

pub fn print_cargo_check_fail() {
    let message =
        "Automatic upgrade failed to fix all issues. Fix them manually, make `cargo check` pass, then continue automatic upgrade!"
        .red();
    println!("\n{message}");
}
