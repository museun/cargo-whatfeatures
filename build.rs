fn main() {
    println!(
        "cargo:rustc-env=WHATFEATURES_USER_AGENT={}",
        format!(
            "whatfeatures/{} (github.com/museun/whatfeatures)",
            get_tag().unwrap().trim()
        )
    );
}

fn get_tag() -> Option<String> {
    let out = std::process::Command::new("git")
        .args(&["describe", "--tags", "--abbrev=0"])
        .output()
        .ok()?
        .stdout;

    String::from_utf8(out).ok()
}
