use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileItem {
    relative_url: String,
    file_name: String,
    file_type: FileType,
}

#[derive(Debug, Serialize)]
pub struct FileItemInTpl {
    relative_url: String,
    file_name: String,
    file_type: String,
    ext: Option<String>,
}

impl FileItem {
    fn new() -> Self {
        Self {
            relative_url: String::new(),
            file_name: String::new(),
            file_type: FileType::Lambda,
        }
    }

    fn into_tpl(&self) -> FileItemInTpl {
        let file_type = match &self.file_type {
            FileType::Folder => "folder",
            FileType::File(_) => "file",
            FileType::Lambda => "lambda",
        };

        let ext = match &self.file_type {
            FileType::File(ext) => Some(ext.clone()),
            _ => None,
        };

        FileItemInTpl {
            relative_url: self.relative_url.clone(),
            file_name: self.file_name.clone(),
            file_type: file_type.to_string(),
            ext,
        }
    }
}


#[derive(Debug, Serialize)]
pub enum FileType {
    Folder,
    File(String),
    Lambda,
}

pub fn list_files_with_type(path: PathBuf, server_path: &PathBuf) -> std::io::Result<Vec<FileItemInTpl>> {
    let mut file_items: Vec<FileItemInTpl> = Vec::new();
    let entries = fs::read_dir(path)?; // 读取目录中的内容

    for entry in entries {
        let mut file_item = FileItem::new();

        let entry = entry?;
        let path = entry.path();

        file_item.file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        println!("path: {:?}", path);
        println!("server path: {:?}", server_path);

        let str = path.strip_prefix(server_path).unwrap().to_str().unwrap().to_string();

        println!("str: {}", str);

        file_item.relative_url = "/".to_owned() + &*str;

        if path.is_dir() {
            file_item.file_type = FileType::Folder;
        } else {
            let file_type = match path.extension().and_then(|s| s.to_str()) {
                Some("jpg") | Some("jpeg") => FileType::File("jpg".to_string()),
                Some("png") => FileType::File("png".to_string()),
                Some("gif") => FileType::File("gif".to_string()),
                Some("svg") => FileType::File("svg".to_string()),
                _ => FileType::File("".to_string()),
            };

            file_item.file_type = file_type;
        }

        file_items.append(&mut vec![file_item.into_tpl()]);
    }

    Ok(file_items)
}


pub fn is_directory(path: PathBuf) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);

    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Invalid path: path not exist or not a directory")
    }
}
