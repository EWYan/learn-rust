use std::{env, fs, process};


fn main() {
    // 获取环境变量LD_SCRIPT_PATH的值，作为链接脚本
    let ld_script_path = match env::var("LD_SCRIPT_PATH") {
        Ok(var) => var,
        _ => process::exit(0),
    };

    // 下面这段代码判断上面环境变量解析出来的文件后缀是否为"ld",if so然后再根据这个文件是否更新来决定是否需要重新编译工程
    let files = fs::read_dir(ld_script_path).unwrap();
    // 从path中获取文件，并通过unwrap返回结果
    // 
    files.filter_map(Result::ok).filter(|d| {
        if let Some(e) = d.path().extension() {
            e == "ld"
        } else {
            false
        }
    } ).for_each(|f| println!("cargo:rerun-if-changed={}", f.path().display()));
}