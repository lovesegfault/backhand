use std::fs::File;
use std::path::PathBuf;

use backhand::{FilesystemReader, FilesystemWriter};
use clap::Parser;

/// tool to replace files in squashfs filesystems
#[derive(Parser, Debug)]
#[command(author, version, name = "replace-backhand")]
struct Args {
    /// Squashfs input image
    filesystem: PathBuf,

    /// Path of file to read, to write into squashfs
    file: PathBuf,

    /// Path of file replaced in image
    file_path: PathBuf,

    /// Squashfs output image
    #[clap(short, long, default_value = "replaced.squashfs")]
    out: PathBuf,
}

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    // read of squashfs
    let file = File::open(args.filesystem).unwrap();
    let filesystem = FilesystemReader::from_reader(file).unwrap();
    let mut filesystem = FilesystemWriter::from_fs_reader(&filesystem).unwrap();

    // Modify file
    let new_file = File::open(&args.file).unwrap();
    filesystem.replace_file(args.file_path, new_file).unwrap();

    // write new file
    let mut output = File::create(&args.out).unwrap();
    filesystem.write(&mut output).unwrap();
    println!("replaced file and wrote to {}", args.out.display());
}
