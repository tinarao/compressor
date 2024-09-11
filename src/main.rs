use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process, u64,
};

use zip::{result::ZipError, write::FileOptions, CompressionMethod, ZipWriter};

// format: ./compressor <target file> <output archive name>
//         ./compressor ./src/main.rs ./src/main.zip

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Format: cargo run <target_file> <output_archive_name>");
        process::exit(0);
    }

    let target_path = String::from(&args[1]);
    let output_name = String::from(&args[2]);

    let _ = compress(target_path, output_name);
}

fn compress(target_path: String, output_name: String) -> Result<(), ZipError> {
    let exists = Path::new(&target_path);
    if !exists.is_file() {
        panic!("you fool")
    }

    let archive_name = format!("{output_name}.zip");

    let zip_file_path = Path::new(archive_name.as_str());
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
