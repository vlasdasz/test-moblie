use std::{
    fmt::Display,
    fs::{copy, create_dir, read_dir, read_to_string, remove_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use convert_case::{Case, Casing};
use git2::Repository;

fn copy_file(names: &Names, src: &Path, dest: &Path) -> Result<()> {
    if src.to_string_lossy().contains(".DS_Store") {
        return Ok(());
    }

    let Ok(content) = read_to_string(src) else {
        copy(src, dest)?;
        return Ok(());
    };

    let content = names.replace_string(content);

    let mut file = File::create(dest)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn copy_dir(names: &Names, src: &Path, dest: &Path) -> Result<()> {
    if !dest.exists() {
        create_dir(dest)?;
    }
    for entry in read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name()).display().to_string();

        if dest_path.contains("xcuserdata") {
            continue;
        }

        let dest_path: PathBuf = names.replace_string(dest_path).into();

        if src_path.is_dir() {
            copy_dir(names, &src_path, &dest_path)?;
        } else {
            copy_file(names, &src_path, &dest_path)?;
        }
    }
    Ok(())
}

const SNAKE_REPLACE: &str = "TEST_MOBILE_PROJECT_NAME_SNAKE_CASE";
const CAMEL_REPLACE: &str = "TEST_MOBILE_PROJECT_NAME_CAMEL_CASE";
const LIB_REPLACE: &str = "TEST_MOBILE_LIB_NAME";

const REPO: &str = "https://github.com/vlasdasz/test-moblie";
const REPO_TEMP: &str = "_test_mobile_temp";

#[derive(Debug)]
struct Names {
    camel: String,
    snake: String,
    lib:   String,
}

impl Names {
    fn replace_string(&self, string: impl Display) -> String {
        let string = format!("{string}");
        let string = string.replace(LIB_REPLACE, &self.lib);
        let string = string.replace(SNAKE_REPLACE, &self.snake);
        let string = string.replace(CAMEL_REPLACE, &self.camel);
        string
    }
}

fn clone_repo(repo_url: &str, dest_path: &str) -> Result<()> {
    let repo_path = Path::new(dest_path);
    if repo_path.exists() {
        remove_dir_all(repo_path)?;
    }
    Repository::clone(repo_url, repo_path)?;
    Ok(())
}

struct TempDir {
    path: &'static str,
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = remove_dir_all(self.path);
    }
}

fn main() -> Result<()> {
    let project_name = read_to_string(".te")
        .or_else(|_| bail!("Please put \'.te' file with project name at the project root."))?;

    let temp_dir = TempDir { path: REPO_TEMP };

    clone_repo(REPO, REPO_TEMP)?;

    let _ = remove_dir_all("mobile");

    let names = Names {
        camel: project_name.to_case(Case::UpperCamel),
        snake: project_name.to_case(Case::Snake),
        lib:   format!("lib{}.a", project_name.to_case(Case::Snake)),
    };

    let src = Path::new(REPO_TEMP).join("mobile-template");
    let dest = Path::new("mobile");

    copy_dir(&names, &src, dest)?;

    let app_icon_path = PathBuf::from("Assets/AppIcon.appiconset");

    if !app_icon_path.exists() {
        return Ok(());
    }

    let target_app_icon_path = PathBuf::from(format!(
        "mobile/iOS/{}/Assets.xcassets/AppIcon.appiconset",
        names.camel
    ));

    let _ = remove_dir_all(&target_app_icon_path);

    copy_dir(&names, &app_icon_path, &target_app_icon_path)?;

    drop(temp_dir);

    println!("iOS project {} generated successfully.", names.camel);

    Ok(())
}
