// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;
use std::process::Command;

#[tauri::command]
fn launch_minecraft() -> Result<String, String> {
    let launcher_dir = "C:/AirDLC";
    let version = "fabric-loader-0.16.9-1.21.4";
    
    let java_path = match find_java() {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    
    let version_dir = format!("{}/versions/{}", launcher_dir, version);
    let natives_dir = format!("{}/natives", version_dir);
    let version_jar = format!("{}/{}.jar", version_dir, version);
    let assets_dir = format!("{}/assets", launcher_dir);
    let libraries_dir = format!("{}/libraries", launcher_dir);
    
    if !Path::new(&version_dir).exists() {
        return Err(format!("Папка версии не найдена: {}", version_dir));
    }
    if !Path::new(&version_jar).exists() {
        return Err(format!("Основной файл версии не найден: {}", version_jar));
    }
    if !Path::new(&assets_dir).exists() {
        return Err(format!("Папка assets не найдена: {}", assets_dir));
    }
    if !Path::new(&libraries_dir).exists() {
        return Err(format!("Папка libraries не найдена: {}", libraries_dir));
    }
    if !Path::new(&natives_dir).exists() {
        return Err(format!("Папка natives не найдена: {}", natives_dir));
    }
    
    let mut classpath = version_jar.clone();
    
    fn collect_jars(dir: &Path, classpath: &mut String) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |e| e == "jar") {
                    classpath.push(';');
                    classpath.push_str(path.to_str().unwrap_or(""));
                } else if path.is_dir() {
                    collect_jars(&path, classpath);
                }
            }
        }
    }
    collect_jars(Path::new(&libraries_dir), &mut classpath);
    
    let mods_dir = format!("{}/mods", launcher_dir);
    if let Ok(entries) = fs::read_dir(&mods_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "jar") {
                classpath.push(';');
                classpath.push_str(path.to_str().unwrap_or(""));
            }
        }
    }
    
    // Записываем classpath в файл, чтобы обойти ограничение длины командной строки
    let temp_dir = std::env::temp_dir();
    let cp_file = temp_dir.join("classpath.txt");
    fs::write(&cp_file, &classpath).map_err(|e| e.to_string())?;
    
    println!("📋 Classpath записан в: {}", cp_file.display());
    
    let mut args = Vec::new();
    args.push("-Xmx2G".to_string());
    args.push("-Djava.library.path=".to_owned() + &natives_dir);
    args.push("-cp".to_string());
    args.push(format!("@{}", cp_file.to_str().unwrap_or("")));
    args.push("net.fabricmc.loader.launch.knot.KnotClient".to_string());
    args.push("--username".to_string());
    args.push("Player".to_string());
    args.push("--version".to_string());
    args.push(version.to_string());
    args.push("--gameDir".to_string());
    args.push(launcher_dir.to_string());
    args.push("--assetsDir".to_string());
    args.push(assets_dir);
    args.push("--assetIndex".to_string());
    args.push("5".to_string());
    args.push("--uuid".to_string());
    args.push("00000000-0000-0000-0000-000000000000".to_string());
    args.push("--accessToken".to_string());
    args.push("0".to_string());
    
    println!("🚀 Запуск Minecraft...");
    println!("📁 Папка: {}", launcher_dir);
    println!("📦 Версия: {}", version);
    println!("🔧 Java: {}", java_path);
    
    match Command::new(&java_path).args(&args).spawn() {
        Ok(child) => {
            let pid = child.id();
            println!("✅ Процесс запущен с PID: {}", pid);
            Ok(format!("Minecraft запущен! PID: {}", pid))
        },
        Err(e) => {
            eprintln!("❌ Ошибка запуска: {}", e);
            Err(format!("Ошибка запуска: {}", e))
        }
    }
}

#[tauri::command]
fn init_launcher() -> Result<String, String> {
    let launcher_dir = "C:/AirDLC";
    let path = Path::new(launcher_dir);

    if !path.exists() {
        fs::create_dir_all(path).map_err(|e| e.to_string())?;
        
        let dirs = ["versions", "libraries", "assets", "mods", "config"];
        for dir in dirs {
            fs::create_dir_all(path.join(dir)).map_err(|e| e.to_string())?;
        }
        
        return Ok("Папка создана".to_string());
    }
    Ok("Папка уже существует".to_string())
}

fn find_java() -> Result<String, String> {
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        let path = format!("{}/bin/javaw.exe", java_home);
        if Path::new(&path).exists() {
            return Ok(path);
        }
    }
    
    let java_paths = [
        "C:/Program Files/Java/jdk-21/bin/javaw.exe",
        "C:/Program Files/Java/jdk-17/bin/javaw.exe",
        "C:/Program Files/Java/jre-17/bin/javaw.exe",
        "C:/Program Files/Java/jdk-11/bin/javaw.exe",
        "C:/Program Files/Java/jre-8/bin/javaw.exe",
    ];
    
    for path in java_paths {
        if Path::new(path).exists() {
            return Ok(path.to_string());
        }
    }
    
    Err("Java не найдена! Установите Java 17 или 21.".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_minecraft, init_launcher])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}