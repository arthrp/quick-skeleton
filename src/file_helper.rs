use std::path::Path;
use std::fs;
use std::io::prelude::*;
use zip::read::ZipFile;
use std::collections::BTreeMap;
use handlebars::Handlebars;
use std::path::PathBuf;
use std::path::Component::ParentDir;

pub fn write_file(file: &mut ZipFile, outpath: &Path, data: &BTreeMap<String,String>) {
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).unwrap();
    let mut outfile = fs::File::create(&outpath).unwrap();

    let mut handlebars = Handlebars::new();
    assert!(handlebars.register_template_string("t1", file_contents).is_ok());

    let res = handlebars.render("t1", data).unwrap();
    outfile.write_all(&res.as_bytes());
}

pub fn create_directory(outpath: &Path) -> () {
    fs::create_dir_all(&outpath).unwrap();
}

pub fn sanitize_filename(filename: &str) -> PathBuf {
    let no_null_filename = match filename.find('\0') {
        Some(index) => &filename[0..index],
        None => filename,
    };

    return Path::new(no_null_filename)
        .components()
        .filter(|component| *component != ParentDir)
        .fold(PathBuf::new(), |mut path, ref cur| {
            path.push(cur.as_os_str());
            path
        });
}
