use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if cfg!(target_os = "windows") {
        copy_winlib();
    }
}

fn copy_winlib() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_lib_path = Path::new(&out_dir).join("../../../deps").join("portaudio.lib");
    let dest_dll_path = Path::new(&out_dir).join("../../..").join("portaudio.dll");
    let _ = fs::copy("./lib/win/portaudio_x64/portaudio.dll", dest_dll_path);
    let _ = fs::copy("./lib/win/portaudio_x64/portaudio.lib", dest_lib_path);
}
