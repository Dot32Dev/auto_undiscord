// For scanning folders for strings
use std::fs;
use std::path::Path;
use std::io::BufRead;

// For downloading the image
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::io::Write;

fn main() {
    println!("Enter the folder path to undiscord:");
    let mut folder_path = String::new();
    std::io::stdin().read_line(&mut folder_path).expect("Failed to read line");
    let folder_path = folder_path.trim();

    let links = scan_folder(folder_path);
    
    // Print the number of links found
    println!("{} Discord CDN links found", links.len());

    // Download the file behind every link
    for link in &links {
        let _ = download_image(link.as_str(), folder_path);
    }
}

fn scan_folder(folder_path: &str) -> Vec<String> {
    let mut found_links = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let links_in_file = search_file(&path);
                    found_links.extend(links_in_file);
                } else if path.is_dir() {
                    let links_in_subfolder = scan_folder(path.to_str().unwrap());
                    found_links.extend(links_in_subfolder);
                }
            }
        }
    }

    found_links
}

fn search_file(file_path: &Path) -> Vec<String> {
    let mut found_links = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        // let file_contents: Vec<String> = lines.map(|line| line.unwrap()).collect();
        let mut updated_file_contents = Vec::new();

        let mut updated = false;
        
        for line in lines {
            if let Ok(line_content) = line {
                let mut updated_line = line_content.clone();

                if let Some(start_idx) = line_content.find("https://cdn.discordapp.com") {
                    if let Some(end_idx) = line_content[start_idx..].find('"') {
                        let link = line_content[start_idx..start_idx + end_idx].to_string();
                        // println!("Link: {}", link);
                        // let _ = download_image(link);
                        found_links.push(link.clone());
                        
                        // Extract the file name from the link
                        let file_name = extract_file_name(&link);
                        // Replace the URL in the line with the extracted file name
                        updated_line.replace_range(start_idx..start_idx + end_idx, format!("/images/{}", &file_name).as_str());
                        updated = true;
                    }
                }
                updated_file_contents.push(updated_line);
            }
        }
        
        // Write the updated contents back to the file
        if updated {
            if let Ok(mut file) = fs::File::create(file_path) {
                for line in updated_file_contents {
                    writeln!(file, "{}", line).expect("Failed to write line");
                }
            }
        }
    }

    found_links
}

fn read_lines(file_path: &Path) -> Result<impl Iterator<Item = Result<String, std::io::Error>>, std::io::Error> {
    let file = fs::File::open(file_path)?;
    Ok(std::io::BufReader::new(file).lines().map(|line| line))
}

fn download_image(url: &str, folder_path: &str) -> Result<(), Box<dyn Error>> {
    // Send a GET request to the URL
    let response = reqwest::blocking::get(url)?;

    // Ensure the request was successful
    if !response.status().is_success() {
        return Err("Failed to download the image".into());
    }

    // Get the image data
    let image_data = response.bytes()?;

    // Extract the filename from the URL
    let filename = match url.rfind('/') {
        Some(index) => format!("{}/public/images/{}", folder_path, &url[index + 1..]),
        None => return Err("No filename found".into()), // ran if no slash is found
    };

    // Create a new file to save the image
    let mut output_file = File::create(filename.clone())?;

    // Write the image data to the file
    copy(&mut image_data.as_ref(), &mut output_file)?;

    println!("Downloaded {}", filename);

    Ok(())
}

fn extract_file_name(url: &str) -> &str {
    // Extract the file name from the URL
    url.rsplit('/').next().unwrap_or("")
}