use colored::Colorize;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
    u64,
};
use zip::{result::ZipError, write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

fn main() {
    let mut command = String::new();
    let stdin = io::stdin();
    println!(
        "Commands:\n\t{} -> compress\n\t{} -> decompress",
        "\"c\"".blue(),
        "\"d\"".red()
    );

    command.clear();
    let _ = stdin.read_line(&mut command);

    if command.trim() == String::from("c") {
        let mut target_file = String::new();
        let mut output_name = String::new();

        println!("{}", "which file do you wanna compress?".blue());
        let _ = stdin.read_line(&mut target_file);

        println!("{}", "how do you wanna name the archive?".blue());
        let _ = stdin.read_line(&mut output_name);

        let target_file = String::from(target_file.trim());
        let output_name = String::from(output_name.trim());

        let _ = compress(target_file, output_name);

        return;
    } else if command.trim() == String::from("d") {
        let mut target_file = String::new();
        let mut output_name = String::new();

        println!("{}", "which file do you wanna decompress?".red());
        let _ = stdin.read_line(&mut target_file);
        if !target_file.trim().ends_with(".zip") {
            println!("{}", "selected file is not .zip archive".on_red());
            return;
        }

        println!("{}", "where to extract?".red());
        let _ = stdin.read_line(&mut output_name);

        let target_file = String::from(target_file.trim());
        let output_name = String::from(output_name.trim());

        let _ = unzip(target_file, output_name);
        return;
    }

    panic!("unknown command.")
}

fn compress(target_path: String, output_name: String) -> Result<(), ZipError> {
    let exists = Path::new(&target_path);
    if !exists.exists() {
        panic!("you fool")
    }

    let started = Instant::now();
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

    println!("Finished in {} sec.", started.elapsed().as_secs());

    Ok(())
}

fn unzip(archive_path: String, target_dir: String) -> Result<(), ZipError> {
    let zip_file_path = Path::new(&archive_path);
    let zip_file = File::open(zip_file_path)?;

    let started = Instant::now();
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

    println!(
        "Finished in {}\nExtracted to {}",
        started.elapsed().as_secs(),
        extract_to.as_os_str().to_str().unwrap()
    );

    Ok(())
}
