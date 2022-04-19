type Config = (&'static str, &'static [&'static str]);
type Configs = &'static [(&'static str, Config)];
const CONFIGS: Configs = &[
    ("clyde", ("./clyde", &[])),
    ("package.json", ("npm", &["run"])),
    ("Makefile", ("make", &[])),
    ("Cargo.toml", ("cargo", &["run"])),
    ("binding.gyp", ("node-gyp", &[])),
    ("gradlew", ("./gradlew", &["run"])),
    ("stack.yaml", ("stack", &[])),
];

fn check(p: &std::path::PathBuf) -> Option<&'static Config> {
    for (name, c) in CONFIGS {
        if p.join(name).exists() {
            return Some(c);
        }
    }
    None
}

fn run(c: &Config) {
    let output = std::process::Command::new(c.0)
        .args(c.1)
        .args(std::env::args().skip(1))
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
    if let Some(s) = output.status.code() {
        std::process::exit(s);
    }
    if !output.status.success() {
        std::process::exit(1);
    }
}

fn main() {
    let mut current = std::env::current_dir().unwrap();
    loop {
        if let Some(c) = check(&current) {
            run(c);
            break;
        }
        match current.parent() {
            Some(c) => current = c.to_owned(),
            None => {
                eprintln!("Unable to find config!");
                eprintln!(
                    "Supported configs:\n{}",
                    CONFIGS
                        .iter()
                        .map(|c| "- ".to_owned() + c.0)
                        .collect::<Vec<String>>()
                        .join("\n")
                );
                std::process::exit(1);
            }
        }
    }
}
