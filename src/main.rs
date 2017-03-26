mod models;
mod file_helper;

use std::env;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use zip::read::ZipFile;
use rustc_serialize::json::{Json};
use rustc_serialize::json;
use std::collections::BTreeMap;
use handlebars::Handlebars;
use models::TemplateParameter;
use file_helper::*;

extern crate zip;
extern crate rustc_serialize;
extern crate handlebars;
#[macro_use] extern crate text_io;

fn main() {
    let args : Vec<String> = env::args().collect();

    if (env::args().count() < 3){
        println!("Wrong arguments supplied.");
        print_usage(&args[0]);
        return;
    }

    let file_path = Path::new(&args[2]);
    let zip_file = File::open(&file_path).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    let params_json : Vec<TemplateParameter> = json::decode(&get_param_json(&mut archive)).unwrap();

    let mut data : BTreeMap<String,String> = fill_data(&params_json);
    extract_content(&mut archive, data);
}

fn extract_content<R: Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>, mut data: BTreeMap<String,String>){
    for i in 0..archive.len(){
        let mut archive_file = archive.by_index(i).unwrap();

        if (archive_file.name() == "parameters.json"){
            continue;
        }

        let write_path = sanitize_filename(archive_file.name());
        create_directory(write_path.parent().unwrap_or(Path::new("")));

        if (&*archive_file.name()).ends_with("/") {
            create_directory(&write_path);
        }
        else {
            write_file(&mut archive_file, &write_path, &data);
        }
    }
}

fn get_param_json<R: Read + Seek>(archive: &mut zip::ZipArchive<R>) -> String
{
    let mut param_file = match archive.by_name("parameters.json"){
        Ok(file) => file,
        Err(..) => {
            println!("File is not valid template");
            std::process::exit(2);
        }
    };

    let mut param_file_contents = String::new();
    param_file.read_to_string(&mut param_file_contents).unwrap();

    return param_file_contents;
}

fn fill_data(params_json: &Vec<TemplateParameter>) -> BTreeMap<String,String>{
    let mut data = BTreeMap::new();
    let mut input = String::new();

    for i in 0..params_json.len(){
        println!("{}:", params_json[i].desc);
        input = read!("{}\n");
        &data.insert(format!("{}", params_json[i].name), format!("{}", input));
    }

    return data;
}

fn print_usage(name: &String) -> (){
    println!("Usage: {0} -c [path to template]", name);
}
