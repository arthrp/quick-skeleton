use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Seek;
use zip::write::FileOptions;
use walkdir::DirEntry;

pub fn zip_dir<T>(walkdir_it: &mut dyn Iterator<Item=DirEntry>, prefix: &str, writer: T, folder_name: &str) -> zip::result::ZipResult<()>
    where T: Write+Seek
{
    let empty_params = b"[]";
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Stored)
    .unix_permissions(0o755);

    let mut buffer = Vec::new();
    zip.start_file("parameters.json", FileOptions::default());
    zip.write_all(empty_params);

    zip.add_directory(format!("{}/", folder_name), FileOptions::default());

    for entry in walkdir_it {
    let path = entry.path();
    let name = path.strip_prefix(Path::new(prefix)).unwrap().to_str().unwrap();

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
    Result::Ok(())
}