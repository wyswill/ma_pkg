use std::{
    env::current_dir,
    fs, future,
    path::{Path, PathBuf},
};

use clap::builder::Str;

/// 检测当前目录是否是monorepo
pub fn check_is_monorepo() -> bool {
    get_pkg_json();
    return false;
}

fn get_pkg_json() {
    let work_space = get_work_space();
    let pkg_path = work_space.join(Path::new("./package.json"));
    let read_rs = fs::read(pkg_path).expect("读取package.json 失败");
    println!("{}", String::from_utf8(read_rs).unwrap());
}

pub fn get_work_space() -> PathBuf {
    let current_work_space = current_dir().expect("获取workspace 失败");
    // println!("The current directory is {}", current_work_space.display());
    current_work_space
}
