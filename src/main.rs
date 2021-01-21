extern crate clap;
extern crate reqwest;

use clap::{App, Arg, SubCommand};
use reqwest::multipart;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("fs-store")
        .version("0.1.0")
        .about("file server commandline client")
        .author("Vivek Pandya");

    let list_option = SubCommand::with_name("list-files").about("List stored files from server");

    let upload_option = SubCommand::with_name("upload-file")
        .about("Upload file from given path to server")
        .arg(
            Arg::with_name("upload-path")
                .help("the path of file to be uploaded")
                .index(1),
        );

    let delete_option = SubCommand::with_name("delete-file")
        .about("Delete stored file with given name from server")
        .arg(
            Arg::with_name("delete-path")
                .help("the path of file to be deleted")
                .index(1),
        );

    let app = app.subcommand(list_option);
    let app = app.subcommand(upload_option);
    let app = app.subcommand(delete_option);

    let matches = app.get_matches();
    match matches.subcommand() {
        ("list-files", _) => {
            let client = reqwest::Client::new();
            let url = format!("http://localhost:8080/files/v1");
            let response = client.get(&url).send();
            println!("{}", response?.text()?);
        }
        ("upload-file", Some(upload_path)) => match upload_path.value_of("upload-path") {
            Some(path_str) => {
                let path = Path::new(path_str);
                let client = reqwest::Client::new();
                let url = format!(
                    "http://localhost:8080/files/v1/{}",
                    path.file_name().unwrap().to_str().unwrap()
                );
                let attachment = multipart::Form::new().file("file", path)?;
                let response = client.post(&url).multipart(attachment).send();
                println!("{}", response?.text()?);
            }
            None => {
                println!("Provide valid path with 'upload-file' subcommand");
            }
        },
        ("delete-file", Some(delete_path)) => match delete_path.value_of("delete-path") {
            Some(path) => {
                let url = format!("http://localhost:8080/files/v1/{}", path);
                let client = reqwest::Client::new();
                let response = client.delete(&url).send();
                println!("{}", response?.text()?);
            }
            None => {
                println!("Provide valid filename with 'delete-file' subcommand");
            }
        },
        _ => {
            println!("provide any of 'list-files', 'upload-file', 'delete-file'");
        }
    }
    Ok(())
}
