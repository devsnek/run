type Config = (&'static str, &'static [&'static str]);
type Configs = &'static [(&'static str, Config)];
const CONFIGS: Configs = &[
    ("package.json", ("npm", &["run"])),
    ("Makefile", ("make", &[])),
    ("Cargo.toml", ("cargo", &["run"])),
    ("binding.gyp", ("node-gyp", &[])),
    ("gradlew", ("./gradlew", &["run"])),
];

fn check(p: &std::path::PathBuf) -> Option<&'static Config> {
    for (name, c) in CONFIGS {
        if p.join(name).exists() {
            return Some(c)
        }
    }
    None
}

fn run(c: &Config) {
    std::process::Command::new(c.0)
        .args(c.1)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
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
                eprintln!("Unable to find config");
                std::process::exit(1);
            },
        }
    }
}
