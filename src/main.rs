#![allow(non_snake_case)]
#![windows_subsystem = "windows"]
//serde
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate clipboard;
//thread pool
extern crate threadpool;
use threadpool::ThreadPool; // yay, simple limited pool threads.
use std::sync::Mutex; // shared data protection for access across multiple threads. Implements .lock()
use std::sync::Arc; // Provides shared ownership of value type, Invoking clone produces new instance (allowing multi thread access)

//clip board
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

// utility
use std::result::Result; // Ok() Err() returns values 

// cryptography
use crypto::digest::Digest; // Cryptography

//FS and Path
use std::path::PathBuf;
use std::fs::ReadDir;
use std::io::{Read, Write};
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::fs::OpenOptions;

// webkit
extern crate web_view;
use web_view::*;

mod html;
use html::Html;

fn path_to_string(path: PathBuf) -> String{
    path.to_str().unwrap().to_string() // hack to make paths easier to open
}

fn recursive_dirs(dir: Result<ReadDir, std::io::Error>) -> Vec<String> {
    let mut builder: Vec<String> = vec!(); // builder is vector that will collect files to hash
    match dir { 
        Ok(rdir) =>  {
            for entry in rdir {
                let entry = entry.unwrap();
                //println!("{:?}", path_to_string(entry.path()));
                if entry.path().is_dir(){ // If item is directory 
                    let appender = recursive_dirs(fs::read_dir(entry.path())); // call self
                    for item in appender { 
                        builder.push(item) // append each file to builder
                    }
                } else { // otherwise file is pushed 
                    builder.push(path_to_string(entry.path()));
                }
            }
        }
        
        Err(e) => {println!("{:?}", e);}
    }
    builder // return Vec<String> of files to be hashed
}   

#[derive(Debug, Serialize, Deserialize)]
struct HashTable {
    File: String, 
    Md5: String,
    Whirlpool: String,
    Sha256: String,
    Sha512: String
}

enum Compute{
	Md5,
	Whirlpool,
	Sha256,
    Sha512,
}

struct Hash;
impl Hash {
	fn new() -> Self {
		Hash{}
	}

	fn compute(&self, contents: &Vec<u8>, htype: Compute) -> String {
		match htype {
			Compute::Md5 => {
				let mut crypto = crypto::md5::Md5::new();
				crypto.input(&contents);
				let digest = crypto.result_str();
				format!("{}", &digest)
			}
			Compute::Whirlpool => {
				let mut crypto = crypto::whirlpool::Whirlpool::new();
				crypto.input(&contents);
				let digest = crypto.result_str();
				format!("{}", &digest)
			}
			Compute::Sha256 => {
				let mut crypto = crypto::sha2::Sha256::new();
				crypto.input(&contents);
				let digest = crypto.result_str();
				format!("{}", &digest)
			}
            Compute::Sha512 => {
                let mut crypto = crypto::sha3::Sha3::sha3_512();
                crypto.input(&contents);
                let digest = crypto.result_str();
                format!("{}", &digest)
            }
		}
	}
}

const TABLE: &str = "document.querySelector('#hashtable')";
fn main() -> WVResult{
    let mut session: Vec<HashTable> = vec!();
    let webview = web_view::builder()
    .title("Hasher Gui v0.1.5")
    .content(Content::Html(Html::content().as_str()))
    .size(800, 600)
    .resizable(true)
    .debug(true)
    .user_data(())
    .invoke_handler(|webview, arg|{
    	match arg {
            "copy" => {
                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                    let mut dat = String::new(); 
                    for item in &session {
                        dat.push_str(&serde_json::to_string(&item).unwrap());
                        dat.push_str("\n");
                    }
                    ctx.set_contents(dat).unwrap();
                },
    		"openf" => match webview.dialog().open_file("Select a file to Hash", "")?{
    			Some(path) => {
                    let path = path_to_string(path);
                    let hasher = Hash::new();
                    let mut fContents: Vec<u8> = vec!();
                    File::open(&path).unwrap().read_to_end(&mut fContents).unwrap();
                    let md5 = hasher.compute(&fContents, Compute::Md5);
                    let whirl = hasher.compute(&fContents, Compute::Whirlpool);
                    let sha256 = hasher.compute(&fContents, Compute::Sha256);
                    let sha512 = hasher.compute(&fContents, Compute::Sha512);
                    let row = HashTable {File: path, Md5: md5, Whirlpool: whirl, Sha256: sha256, Sha512: sha512};
                    webview.eval(format!("generateTable({}, [{}])", TABLE, serde_json::to_string(&row).unwrap()).as_str()).unwrap();
                    session.push(row);
                    //webview.dialog().info("File chosen", path)
                    Ok(())
                },
    			None => webview.dialog().warning("Warning", "No File chosen"),
    		}?,
            "opend" => match webview.dialog().choose_directory("Select a file to Hash", "")?{
                Some(path) => {
                    let pool = ThreadPool::new(30);
                    let builder = recursive_dirs(fs::read_dir(path_to_string(path)));
                    let rows: Vec<HashTable> = vec!();
                    let rows = Arc::new(Mutex::new(rows));
                    for p in builder {
                        let rows = rows.clone();
                        pool.execute(move || {
                            //let mut rows = rows.lock().unwrap();
                            let hasher = Hash::new();
                            let path = p;
                            let mut fContents: Vec<u8> = vec!();
                            File::open(&path).unwrap().read_to_end(&mut fContents).unwrap();
                            let md5 = hasher.compute(&fContents, Compute::Md5);
                            let whirl = hasher.compute(&fContents, Compute::Whirlpool);
                            let sha256 = hasher.compute(&fContents, Compute::Sha256);
                            let sha512 = hasher.compute(&fContents, Compute::Sha512);
                            let mut rows = rows.lock().unwrap();
                            rows.push(HashTable {File: path, Md5: md5, Whirlpool: whirl, Sha256: sha256, Sha512: sha512});
                        });
                    }
                        pool.join();
                        for row in rows.lock().unwrap().iter() {
                            webview.eval(format!("generateTable({}, [{}])", TABLE, serde_json::to_string(&row).unwrap()).as_str()).unwrap();
                        }
                        session.append(&mut rows.lock().unwrap());
                    Ok(())
                },
                None => webview.dialog().warning("Warning", "No File chosen"),
            }?,
    		"save" => match webview.dialog().save_file()? {
                Some(path) => {
                    let path = path_to_string(path);
                    let mut file = OpenOptions::new().append(true).create(true).open(&path).unwrap();
                    for item in &session {
                        file.write_fmt(format_args!("{}\n", serde_json::to_string(&item).unwrap())).unwrap();
                    }
                    webview.dialog().info("File Saved", path)
                    //Ok(())
                },
                None => webview
                    .dialog()
                    .warning("Warning", "You didn't choose a save location."),
            }?,
    		"load" => match webview.dialog().open_file("Select a prior session to load", "")?  {
                Some(path) => {
                    let mut appender: Vec<HashTable> = vec!(); 
                    let path = path_to_string(path);
                    let mut err_count: u64 = 0;
                    let file = File::open(&path).unwrap();
                    for line in ::std::io::BufReader::new(file).lines() {
                        let dat : Result<HashTable, serde_json::Error>=  serde_json::from_str(&line.unwrap());
                        match dat {
                            Ok(adata) => {appender.push(adata)},
                            Err(_) => {err_count += 1},
                        }
                    }
                    for row in appender {
                        webview.eval(format!("generateTable({}, [{}])", TABLE, serde_json::to_string(&row).unwrap()).as_str()).unwrap();
                        session.push(row);
                    }
                    if err_count > 0 {
                        webview.dialog().info("Warning" ,format!("{} Lines/Entries could not be loaded", err_count))
                    } else {
                        webview.dialog().info("All entries loaded", path)
                    }
                    //Ok(())
                },

                None => webview
                    .dialog()
                    .warning("Warning", "You didn't choose a file."),
            }?,
    		"exit" => webview.exit(),
    		_ => { unimplemented!();
                /*if session.len() != 0 {
                    for i in (0..session.len()).rev() {
                        if session[i].File == arg {
                            session.remove(i);
                        }
                    }
                }*/
            },
    	};
    	Ok(())
    })
    .build()?;
    webview.run()
}