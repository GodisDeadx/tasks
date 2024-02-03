use dirs::home_dir;
use std::fs::{self, File};
use std::path::{PathBuf};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use crate::gui::{Tasks};
use std::fs::OpenOptions;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub run: bool,
    pub x: i32,
    pub y: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { run: false, x: 100, y: 100 }
    }
}

pub fn get_path(input: &str) -> PathBuf {
    // get the current users AppData\Local folder on windows
    let mut path = home_dir().unwrap();
    path.push(format!("AppData\\Local\\Tasks\\{}", input));
    path
}

fn check_file_exists(path: &PathBuf) {
    if !path.exists() {
        if path.is_file() {
            File::create(&path).expect("Failed to create file");
        } else {
            fs::create_dir_all(path.parent().unwrap()).expect("Failed to create directory");
            File::create(&path).expect("Failed to create file");
        }
    }
}

fn create_dir() {
    let path = get_path("");
    fs::create_dir_all(path).expect("Failed to create directory");
}

pub fn read_tasks(name: String) -> Result<Tasks, String> {
    let path_name = name + ".json";
    let path = get_path(&path_name);

    if path.exists() {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open file".to_string()),
        };

        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        match reader.read_to_string(&mut contents) {
            Ok(_) => {
                if contents.is_empty() {
                    Ok(Tasks { tasks: vec![] })
                } else {
                    match serde_json::from_str(&contents) {
                        Ok(tasks) => Ok(tasks),
                        Err(err) => Err(format!("Failed to parse JSON: {}", err)),
                    }
                }
            },
            Err(err) => Err(format!("Failed to read file: {}", err)),
        }
    } else {
        Ok(Tasks { tasks: vec![] })
    }
}

pub fn write_task(task_list: &Tasks, list_name: String) {
    let name = list_name.clone() + ".json";
    let path = get_path(&name);
    check_file_exists(&path);

    let mut tasks = read_tasks(list_name.clone()).unwrap_or_else(|_| Tasks { tasks: vec!() });

    let mut unique_ids: HashSet<i32> = HashSet::new();

    for task in &task_list.tasks {
        if let Some(existing_task) = tasks.tasks.iter_mut().find(|t| t.id == task.id) {
            if existing_task.name != task.name || existing_task.description != task.description || existing_task.completed != task.completed || existing_task.tags != task.tags {
                *existing_task = task.clone();
            }
        } else {
            tasks.tasks.push(task.clone());
        }
        unique_ids.insert(task.id);
    }

    let json_str = serde_json::to_string_pretty(&tasks).unwrap_or_else(|_| String::from("Failed to serialize tasks"));();

    let mut file = OpenOptions::new().write(true).truncate(true).open(&path).expect("Failed to open/write file");
    file.write_all(json_str.as_bytes()).expect("Failed to write to file");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowPosition {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

pub fn write_settings(state: bool, pos: WindowPosition) {
    let path = get_path("settings.json");
    check_file_exists(&path);

    let mut settings = read_settings().unwrap_or(Settings { run: false, x: 100, y: 100 });
    settings.run = state;
    settings.x = pos.x;
    settings.y = pos.y;

    let mut file = OpenOptions::new().write(true).open(&path).expect("Failed to open/write file");
    let json_str = serde_json::to_string_pretty(&settings);
    file.set_len(0).expect("Failed to truncate file");
    file.seek(SeekFrom::Start(0)).expect("Failed to seek to start of file");
    file.write_all(json_str.unwrap().as_bytes()).expect("Failed to write to file");
}

pub fn read_settings() -> Result<Settings, String> {
    let path = get_path("settings.json");

    if !path.exists() {
        create_dir();
        write_settings(false, WindowPosition { x: 100, y: 100 });
    }

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open file".to_string()),
    };

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    match reader.read_to_string(&mut contents) {
        Ok(_) => {
            match serde_json::from_str(&contents) {
                Ok(settings) => Ok(settings),
                Err(err) => Err(format!("Failed to parse JSON: {}", err)),
            }
        },
        Err(err) => Err(format!("Failed to read file: {}", err)),
    }
}

pub fn delete_tasks(id: i32, file_name: String) {
    let mut tasks = read_tasks(file_name.clone()).unwrap_or(Tasks { tasks: vec!() });

    tasks.tasks.retain(|task| task.id != id);

    // Reassign IDs based on current index
    for (index, task) in tasks.tasks.iter_mut().enumerate() {
        task.id = index as i32;
    }

    let name = file_name.clone() + ".json";
    let path = get_path(&name);

    let json_str = serde_json::to_string_pretty(&tasks);
    let mut file = OpenOptions::new().write(true).open(&path).expect("Failed to open/write file");
    file.set_len(0).expect("Failed to truncate file");
    file.seek(SeekFrom::Start(0)).expect("Failed to seek to start of file");
    file.write_all(json_str.unwrap().as_bytes()).expect("Failed to write to file");
}

pub fn create_new_task_file(name: String) {
    let path_name = name + ".json";
    let path = get_path(&path_name);
    File::create(&path).expect("Failed to create file");
}

pub fn delete_task_file(name: String) {
    let path_name = name + ".json";
    let path = get_path(&path_name);
    fs::remove_file(&path).expect("Failed to delete file");
}

pub fn get_files(directory_path: &str) -> Result<Vec<String>, String> {
    let path = get_path("");
    let dir = match fs::read_dir(&path) {
        Ok(dir) => dir,
        Err(err) => return Err(format!("Failed to read directory: {}", err)),
    };

    let mut file_names = Vec::new();
    for entry in dir {
        if let Ok(entry) = entry {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name != "settings.json" {
                        file_names.push(file_name.to_string());
                    }
                }
            }
        }
    }

    Ok(file_names)
}