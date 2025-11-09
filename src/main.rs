use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};
use std::process::{Command, Stdio};

struct Parameters {
    fps: u8,
    buffer_time: u8,
    quality: String,
    sound_source: String,
    output_directory: PathBuf
}

fn param_values() -> Parameters {
    // sound_source="$(pactl info | grep 'Default Sink' | awk '{print $3;}').monitor"
    let sound_source = Command::new("sh")
        .arg("-c")
        .arg(r#"pactl info | grep 'Default Sink' | awk '{print $3}'"#)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok()) // and_then... this dork ass language lmfao
        .as_deref()
        .map(|s| format!("{}.monitor", s.trim()))
        .unwrap_or(":".to_string());

    // I miss the old Kanye
    Parameters {
        fps: 60,
        buffer_time: 90,
        quality: "ultra".into(),
        sound_source,
        output_directory: Path::new("/mnt/recordings/clips").to_path_buf() // make sure to change
                                                                           // this to a path you
                                                                           // actually want
    }
}

// Check gpu-screen-recorder is actually in $PATH
fn buy_a_new_crib_like_it_aint_nun(cmd: &str) -> bool {
    // Imagine using a crate 🫵 🤣🖕 
    Command::new(cmd)
        .stdin(Stdio::null()) 
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| true)
        .unwrap_or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                false
            } else {
                eprintln!("Error checking '{}': {}", cmd, e);
                false
            }
        })
}

// Why don't you make these just one function?
// Because im TRILL AS FUCK _|_   _|_ <---- These are my gargantuan fists
fn check_gpu_screen_recorder() {
    if buy_a_new_crib_like_it_aint_nun("gpu-screen-recorder") {
        println!("Found gpu-screen-recorder!")
    } else {
        println!("Install gpu-screen-recorder you dunce.")
    }
}

fn check_output_directory() -> std::io::Result<()> {
    let output_dir = &param_values().output_directory;

    if output_dir.is_dir() {
        return Ok(());
    }

    println!("Creating directory: {}", output_dir.display());
    fs::create_dir_all(output_dir)
}

fn launch_gpu_screen_recorder(params: &Parameters) -> std::io::Result<u32> {
    let out_dir = params.output_directory.display().to_string();

    // Command::new() didn't work 
    let cmd = format!(
        "gpu-screen-recorder -w screen -c mp4 -f {} -q {} -a '{}' -o '{}' -r {}",
        params.fps, params.quality, params.sound_source, out_dir, params.buffer_time
    );

    let child = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let pid = child.id();

    notify_send("Replay buffer started", &format!("Saving to {out_dir}"));

    Ok(pid)

}

fn send_signal(pid: u32, sig: &str) -> std::io::Result<()> {
     Command::new("kill")
        .arg(format!("-{sig}"))
        .arg(pid.to_string())
        .status()
        .map(|_| ())
}

fn notify_send(title: &str, msg: &str) {
    let _ = Command::new("notify-send")
        .arg(title)
        .arg(msg)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

fn main() -> std::io::Result<()> {
    let params = param_values();
    let sr_pid = std::process::id();

    check_gpu_screen_recorder();
    check_output_directory().unwrap_or_else(|e| {
            eprintln!("Failed to create output_directory: {}", e);
            std::process::exit(1);
        });
    let record_pid = launch_gpu_screen_recorder(&params).unwrap_or_else(|e| {
        eprintln!("Failed to launch gpu-screen-recorder!: {}", e);
        std::process::exit(1);
    });

    println!("screen_record at PID: {sr_pid} | record pid: {record_pid}");
    
    // Listen for hyprland pass/kill keybinds 
    for line in io::stdin().lock().lines() {
        let cmd = line?.trim().to_lowercase();
        if cmd == "save" {
            send_signal(record_pid, "SIGUSR1")?;
            notify_send("Saved last {}s", &format!("{}", params.buffer_time));
        }
    }
    Ok(())
}
