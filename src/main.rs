use std::{env, fs, path::Path, process, io::Write, error::Error, io};
use reqwest::{self, Client};
use zip::read::ZipArchive;
use indicatif::{ProgressBar, ProgressStyle};

const FFMPEG_URL: &str = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip";

#[tokio::main]
async fn main() {
    let mut found_ffmpeg: bool = false;
    let mut ffmpeg_path: std::path::PathBuf;
    let extraction_path: &Path = Path::new("C:/Program Files/");
    let zip_file_path: &str = "C:/Program Files/ffmpeg.zip";
    let ffmpeg_exe_path = r"C:\Program Files\ffmpeg\bin";

    //Check if ffmpeg exists and continue to the program if it does.
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            ffmpeg_path = path.join("ffmpeg.exe");

            if ffmpeg_path.is_file() {
                println!("Executable found at {}", ffmpeg_path.display());
                found_ffmpeg = true;
                break;
            }
        }
    };


    //If ffmpeg isn't installed, download and extract it, then add it to path.

    if !found_ffmpeg {
        println!("ffmpeg not found. Do you want to download it now? (Y/n)");

        loop{

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line, please try again.");


            match input.trim().to_uppercase().as_str(){
                "Y" => break,
                "N" => {
                    println!("ffmpeg will not install.");
                    exit_on_user_input();
                },
                &_ => continue,

            };
        }

        println!("Downloading archive..");

        ffmpeg_download().await.unwrap_or_else(|err| {
            println!("Failed to write archive. ({err}). Consider running script as Administrator.");
            exit_on_user_input();
        });

        println!("Decompressing archive..");

        decompress_file(zip_file_path, extraction_path).await.unwrap_or_else(|err|{
            println!("Failed to decompress archive. ({err}). Consider running script as Administrator.");
            exit_on_user_input();
        });

        if let Ok(folders) = fs::read_dir(extraction_path) {
            // Iterate over the entries
            for folder in folders.flatten() {
                if let Some(folder_name) = folder.file_name().to_str() {
                    // Check if the entry starts with "ffmpeg"
                    if folder_name.starts_with("ffmpeg") {
                        let old_path = folder.path();
                        let new_path = folder.path().with_file_name("ffmpeg");

                        // Rename the folder
                        if let Err(err) = fs::rename(old_path, new_path) {
                            eprintln!("Failed to rename folder: {}", err);
                            exit_on_user_input();
                        } else {
                            println!("Folder renamed successfully!");
                            break; // Stop iterating after renaming the first folder
                        }
                    }
                }
            }
        }

        println!("Do you want to add ffmpeg to PATH environment variable? (Recommended) (Y/n)");

        loop{

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line, please try again.");

            //Add ffmpeg to path if Y
            match input.trim().to_uppercase().as_str(){
                "Y" => {
                    let current_path = env::var_os("PATH").unwrap_or_default();
                    let new_path = format!("{};{}", current_path.to_string_lossy(), ffmpeg_exe_path);

                    if let Err(err) = process::Command::new("setx")
                        .args(["/M", "PATH", &new_path])
                        .stdout(process::Stdio::null())
                        .status(){

                        println!("Failed to update the PATH environment variable with error: {err}");
                        exit_on_user_input();
                        }

                    println!("Added `{ffmpeg_exe_path}` to PATH environment variable.");

                }

                "N" => {
                    println!("ffmpeg will not be added to PATH. Execution cannot continue.");
                    exit_on_user_input();
                },

                &_ => continue,
            };
        }

    }
    //Main program starts here, install script just ended.

    run_ffmpeg();
    exit_on_user_input();
}

async fn ffmpeg_download() -> Result<(), Box<dyn Error>> {

    let client = Client::new();
    let mut response = client.get(FFMPEG_URL).send().await?;

    let content_length = response.content_length().unwrap_or(0);
    let progress_bar = ProgressBar::new(content_length);
     progress_bar.set_style(ProgressStyle::default_bar()
         .template("[{bar:40.green/white}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})")?
         .progress_chars("#--"));

    if response.status().is_success() {

        let mut file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(Path::new("C:/Program Files/ffmpeg.zip"))?;

        while let Some(chunk) = response.chunk().await?{

            file.write_all(&chunk)?;
            progress_bar.inc(chunk.len() as u64);

        }
        progress_bar.finish();

    }
    else {
        Err("Failed to download ffmpeg archive. Check your internet connection.")?
    }

    Ok(())
}

async fn decompress_file(archive_path: &str, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    let content_length = archive.len();
    let progress_bar = ProgressBar::new(content_length as u64);

    progress_bar.set_style(ProgressStyle::default_bar()
         .template("[{bar:40.green/white}] {pos}/{len}")?
         .progress_chars("#--"));

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = match file.enclosed_name() {
            Some(name) => output_path.join(name),
            None => continue,
        };

        if file.is_dir() {
            fs::create_dir_all(&file_path)?;
        } else {
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut output_file = fs::File::create(&file_path)?;
            std::io::copy(&mut file, &mut output_file)?;
            progress_bar.inc(1);
        }
    }
    progress_bar.finish();

    println!("Deleting archive..");
    if let Err(err) = fs::remove_file(archive_path){
        println!("Failed to delete archive with err: {err}");
    }

    Ok(())
}

fn exit_on_user_input() {
    println!("Press any key to continue...");
    io::stdout().flush().unwrap();

    let _ = io::stdin().read_line(&mut String::new()).unwrap();

    process::exit(0);
}

fn run_ffmpeg(){
    //command line script here
}
