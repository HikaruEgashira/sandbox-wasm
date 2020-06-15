use std::env;
use std::fs;
use std::io::{Read, Write};

#[macro_use] // macroを使うのでmacro_useを追記
extern crate clap;

use clap::{App, Arg};

fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    let mut input_file =
        fs::File::open(input_fname).map_err(|err| format!("error opening input: {}", err))?;
    let mut contents = Vec::new();
    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("read error: {}", err))?;

    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output '{}': {}", output_fname, err))?;
    output_file
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))
}

fn main() {
    let app = App::new(crate_name!()) // Cargo.tomlのnameを参照する
        .version(crate_version!()) // Cargo.tomlのversionを参照する
        .author(crate_authors!()) // Cargo.tomlのauthorsを参照する
        .about(crate_description!()) // Cargo.tomlのdescriptionを参照する
        .arg(Arg::with_name("path"));

    // 引数を解析
    let matches = app.get_matches();

    // pathが指定されていれば値を表示
    if let Some(o) = matches.value_of("path") {
        ls(o).iter().for_each(|f| println!("{}", f));
    }
}

fn ls(base: &str) -> Vec<String> {
    let dir = fs::read_dir(base).unwrap();

    let mut entry = dir
        .map(|f| f.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    entry.sort();
    let entry = entry; // immutable にする

    entry
}
