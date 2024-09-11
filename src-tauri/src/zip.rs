use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    u64,
};
use zip::{result::ZipError, write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

pub fn unzip(archive_path: String, target_dir: String) -> Result<(), ZipError> {
    let zip_file_path = Path::new(&archive_path);
    let zip_file = File::open(zip_file_path)?;

    let mut archive = ZipArchive::new(zip_file)?;
    let extract_to = Path::new(target_dir.as_str());

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();

        let target_path = extract_to.join(file_name);
        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let mut output_file = File::create(&target_path)?;
        io::copy(&mut file, &mut output_file)?;
    }

    Ok(())
}

pub fn compress(target_path: String, output_name: String) -> Result<(), ZipError> {
    let exists = Path::new(&target_path);
    if !exists.exists() {
        panic!("you fool")
    }

    let op = output_name.as_str();

    let zip_file_path = Path::new(op);
    let zip_file = File::create(&zip_file_path)?;

    let mut zip = ZipWriter::new(zip_file);
    let opts = FileOptions::default().compression_method(CompressionMethod::DEFLATE);

    let file_to_compress = PathBuf::from(target_path);
    let file = File::open(&file_to_compress)?;
    let file_name = file_to_compress.file_name().unwrap().to_str().unwrap();

    let _ = zip.start_file(file_name, opts);
    let mut buf = Vec::new();

    let mut reader = file.take(u64::MAX);
    io::copy(&mut reader, &mut buf)?;

    zip.write_all(&buf)?;
    zip.finish()?;

    Ok(())
}
