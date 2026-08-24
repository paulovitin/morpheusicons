//! Simple static file server for the MorpheusIcons web demo.
//! Serves pages/ at the site root plus the WASM pkg/ directory on localhost:8765
//! with live-reload support.

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let addr = "127.0.0.1:8765";
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");

    println!("🌐 MorpheusIcons Web Demo");
    println!("   Serving at: http://{addr}");
    println!("   Live reload: Enabled (watching WASM, CSS & HTML changes)");
    println!("   Press Ctrl+C to stop.\n");

    // Try to open browser
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(format!("http://{addr}"))
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(format!("http://{addr}"))
            .spawn();
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("Connection error: {e}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf).unwrap_or(0);
    if n == 0 {
        return;
    }
    let request = String::from_utf8_lossy(&buf[..n]);

    // Parse the request path
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    if path == "/__livereload" {
        let response = "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/event-stream\r\n\
                        Cache-Control: no-cache\r\n\
                        Connection: keep-alive\r\n\
                        Access-Control-Allow-Origin: *\r\n\r\n";
        if stream.write_all(response.as_bytes()).is_err() {
            return;
        }

        let mut client_last_time = get_max_mtime();
        loop {
            thread::sleep(Duration::from_millis(300));
            let current_mtime = get_max_mtime();
            if current_mtime > client_last_time {
                client_last_time = current_mtime;
                if stream.write_all(b"data: reload\n\n").is_err() {
                    break;
                }
            } else {
                if stream.write_all(b": ping\n\n").is_err() {
                    break;
                }
            }
        }
        return;
    }

    // Pages live in pages/ and are served at the site root, exactly as they
    // are laid out in _site/. Anything pages/ does not hold (dist/, pkg/,
    // assets/) falls through to the repository root.
    let file_path = match path {
        "/" | "/index.html" => "pages/index.html".to_string(),
        p => {
            let rel = p.trim_start_matches('/').to_string();
            let in_pages = format!("pages/{rel}");
            if Path::new(&in_pages).is_file() {
                in_pages
            } else {
                rel
            }
        }
    };

    let full_path = Path::new(&file_path);

    if full_path.exists() && full_path.is_file() {
        let content = fs::read(full_path).unwrap_or_default();
        let mime = guess_mime(&file_path);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\nCache-Control: no-cache, no-store, must-revalidate\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
            content.len()
        );
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.write_all(&content);
    } else {
        let body = "404 Not Found";
        let response = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        );
        let _ = stream.write_all(response.as_bytes());
    }
}

fn get_max_mtime() -> u64 {
    let files = [
        "pages/index.html",
        "pages/get-started.html",
        "pkg/morpheusicons.js",
        "pkg/morpheusicons_bg.wasm",
        "dist/output.css",
        "src/input.css",
    ];
    let mut max_mtime = 0;
    for file in files {
        if let Ok(metadata) = fs::metadata(file) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(dur) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                    let millis = dur.as_millis() as u64;
                    if millis > max_mtime {
                        max_mtime = millis;
                    }
                }
            }
        }
    }
    max_mtime
}

fn guess_mime(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "application/javascript",
        "wasm" => "application/wasm",
        "css" => "text/css",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "json" => "application/json",
        "ts" => "application/typescript",
        _ => "application/octet-stream",
    }
}
