#![allow(clippy::try_err)]

mod file_helper;
mod models;

use models::TemplateParameter;
use rustc_serialize::json;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Seek;
use std::path::Path;
use text_io::{read, try_read, try_scan};
use walkdir::{DirEntry, WalkDir};
use zip::write::FileOptions;

fn main() {
    let args: Vec<String> = env::args().collect();

    if env::args().count() < 3 {
        println!("Wrong arguments supplied.");
        print_usage(&args[0]);
        return;
    }

    let mode = &args[1];

    if mode == "-c" {
        let file_path = Path::new(&args[2]);
        let zip_file = File::open(&file_path).unwrap();
        let mut archive = zip::ZipArchive::new(zip_file).unwrap();
        let params_json: Vec<TemplateParameter> =
            json::decode(&get_param_json(&mut archive)).unwrap();

        let data: BTreeMap<String, String> = fill_data(&params_json);
        extract_content(&mut archive, &data);
    } else if mode == "-n" {
        let dir_to_zip = &args[2];
        let folder_name = &args[4];
        let file_name = format!("{}.zip", folder_name);
        let zip_file_name = Path::new(&file_name);
        let file = File::create(zip_file_name).unwrap();

        let walkdir = WalkDir::new(dir_to_zip);
        let walkdir_iter = walkdir.into_iter();

        zip_dir(
            &mut walkdir_iter.filter_map(|e| e.ok()),
            dir_to_zip,
            file,
            folder_name,
        )
        .unwrap()
    } else {
        print_usage(&args[0]);
    }
}

fn zip_dir<T>(
    walkdir_iter: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    folder_name: &str,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let empty_params = b"[]";
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    zip.start_file("parameters.json", FileOptions::default())?;
    zip.write_all(empty_params)?;
    zip.add_directory(format!("{}/", folder_name), FileOptions::default())?;

    for entry in walkdir_iter {
        let path = entry.path();
        let name = path
            .strip_prefix(Path::new(prefix))
            .unwrap()
            .to_str()
            .unwrap();

        if path.is_file() {
            println!("adding {:?} as {:?} ...", path, name);
            zip.start_file(format!("{}/{}", folder_name, name), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        }
    }
    zip.finish()?;
    Ok(())
}

fn extract_content<R: Read + Seek>(
    archive: &mut zip::ZipArchive<R>,
    data: &BTreeMap<String, String>,
) {
    for i in 0..archive.len() {
        let mut archive_file = archive.by_index(i).unwrap();

        if archive_file.name() == "parameters.json" {
            continue;
        }

        let write_path = file_helper::sanitize_filename(archive_file.name());
        file_helper::create_directory(write_path.parent().unwrap_or_else(|| Path::new("")));

        if (&*archive_file.name()).ends_with('/') {
            file_helper::create_directory(&write_path);
        } else {
            file_helper::write_file(&mut archive_file, &write_path, &data);
        }
    }
}

fn get_param_json<R: Read + Seek>(archive: &mut zip::ZipArchive<R>) -> String {
    let mut param_file = match archive.by_name("parameters.json") {
        Ok(file) => file,
        Err(..) => {
            println!("File is not a valid template");
            std::process::exit(2);
        }
    };

    let mut param_file_contents = String::new();
    param_file.read_to_string(&mut param_file_contents).unwrap();

    param_file_contents
}

fn fill_data(params_json: &[TemplateParameter]) -> BTreeMap<String, String> {
    let mut data = BTreeMap::new();
    let mut input: String;

    for param in params_json {
        println!("{}:", param.desc);
        input = read!("{}\n");
        data.insert(param.name.clone(), input);
    }

    println!("Input project folder name:");
    input = read!("{}\n");
    data.insert("folder_name".to_string(), input);

    data
}

fn print_usage(name: &str) {
    println!(
        "Usage: {0} -c [path to template] - scaffold new project",
        name
    );
    println!(
        "{0} -n [forder path] -f [default project folder name] - create template from folder",
        name
    );
}
