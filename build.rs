use std::process::Command;

fn main() {
    for path in ["package.json", "input.css", "src"] {
        println!("cargo:rerun-if-changed={}", path);
    }
    let status = Command::new("npm").args(["run", "build"]).status();
    if let Ok(s) = status
        && !s.success()
    {
        panic!("Tailwind build failed. Ensure you've run 'npm install'.");
    }
}
