use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pkgjson {
    name: String,
    version: String,
    description: String,
    workspaces: Option<String>,
}

/// 检测当前目录是否是monorepo
pub fn check_is_monorepo() -> bool {
    let pkg = get_pkg_json();
    if pkg.workspaces.is_none() {
        return false;
    } else {
        return true;
    }
}

fn get_pkg_json() -> Pkgjson {
    let work_space = get_work_space();
    let pkg_path = work_space.join(Path::new("./package.json"));
    let read_rs = fs::read(pkg_path).expect("读取package.json 失败");
    let pkg_json =
        serde_json::from_str(&String::from_utf8(read_rs).unwrap()).expect("解析package.json 失败");
    pkg_json
}

pub fn get_work_space() -> PathBuf {
    let current_work_space = current_dir().expect("获取workspace 失败");
    // println!("The current directory is {}", current_work_space.display());
    current_work_space
}
