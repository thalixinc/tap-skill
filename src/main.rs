use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir};
use is_terminal::IsTerminal;
use std::cmp::Ordering;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

static SKILLS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/skills");

const REPO: &str = "https://github.com/thalixinc/tap-skill";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "tap-skill")]
#[command(about = "Traceable Acceptance Pack (TAP): Claims → Use Cases → Acceptance Tests", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print version information")]
    Version {
        #[arg(long, help = "Automatically update if newer version available")]
        yes: bool,
    },
    #[command(about = "Update tap to the latest version")]
    Update {
        #[arg(long, help = "Check for updates without installing")]
        check: bool,
    },
    #[command(about = "Upgrade tap to the latest version (alias for update)")]
    Upgrade {
        #[arg(long, help = "Check for updates without installing")]
        check: bool,
    },
    #[command(about = "Install TAP skill into an agent skill directory")]
    Install {
        #[arg(
            long,
            value_delimiter = ',',
            help = "Harness(es) to install to: cursor, claude, codex, or custom path (comma-separated for multiple)"
        )]
        harness: Option<Vec<String>>,
        #[arg(
            long,
            help = "Scope: user (global) or project (local to cwd). Interactive prompt if not specified."
        )]
        scope: Option<String>,
        #[arg(long, value_name = "DIR", help = "Custom path for 'custom' harness")]
        path: Option<PathBuf>,
    },
    #[command(about = "Show detected skill install locations and status")]
    Doctor,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Version { yes } => cmd_version(yes),
        Commands::Update { check } => cmd_update(check),
        Commands::Upgrade { check } => cmd_update(check),
        Commands::Install {
            harness,
            scope,
            path,
        } => cmd_install(harness, scope, path),
        Commands::Doctor => cmd_doctor(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn cmd_version(yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("tap-skill {}", VERSION);

    let latest = match fetch_latest_version() {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };

    if semver_cmp(&latest, VERSION) != Ordering::Greater {
        return Ok(());
    }

    if yes {
        do_update()?;
        println!("update: tap-skill upgraded {} -> {}", VERSION, latest);
        return Ok(());
    }

    if !io::stdin().is_terminal() {
        println!(
            "update available: {} — run `tap-skill update`, or `tap-skill version --yes` to update now",
            latest
        );
        return Ok(());
    }

    print!("update available: {} — run tap-skill update, or --yes to update now [y/N] ", latest);
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    if matches!(line.trim(), "y" | "Y" | "yes" | "YES") {
        do_update()?;
        println!("update: tap-skill upgraded {} -> {}", VERSION, latest);
    }
    Ok(())
}

fn cmd_update(check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let latest = fetch_latest_version()?;
    let available = semver_cmp(&latest, VERSION) == Ordering::Greater;

    if check {
        println!(
            "package: tap-skill\ncurrent: {}\nlatest: {}\navailable: {}",
            VERSION, latest, available
        );
        if available {
            println!("\nRun `tap-skill update` to upgrade");
        } else {
            println!("\nAlready up to date");
        }
        return Ok(());
    }

    if !available {
        println!("ok: tap-skill already at latest ({})", VERSION);
        return Ok(());
    }

    do_update()?;
    println!("update: tap-skill upgraded {} -> {}", VERSION, latest);
    Ok(())
}

fn do_update() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("cargo")
        .args(["install", "--git", REPO, "--force"])
        .status()?;
    if !status.success() {
        return Err("cargo install failed".into());
    }
    Ok(())
}

fn semver_cmp(a: &str, b: &str) -> Ordering {
    let pa: Vec<u64> = a.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let pb: Vec<u64> = b.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    for i in 0..pa.len().max(pb.len()) {
        match pa
            .get(i)
            .copied()
            .unwrap_or(0)
            .cmp(&pb.get(i).copied().unwrap_or(0))
        {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

fn fetch_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/repos/thalixinc/tap-skill/contents/Cargo.toml";
    let out = Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "10",
            "-H",
            "Accept: application/vnd.github.raw+json",
            url,
        ])
        .output()?;
    if !out.status.success() {
        return Err("could not reach the version source".into());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    text.lines()
        .find_map(|l| {
            let l = l.trim();
            l.strip_prefix("version = \"")
                .and_then(|r| r.strip_suffix('"'))
                .map(String::from)
        })
        .ok_or_else(|| "version not found in remote Cargo.toml".into())
}

fn cmd_install(
    harness_arg: Option<Vec<String>>,
    scope_arg: Option<String>,
    path: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let is_interactive = io::stdin().is_terminal();

    // Determine harnesses
    let harnesses = if let Some(h) = harness_arg {
        h
    } else if is_interactive {
        prompt_harnesses()?
    } else {
        return Err("Non-interactive mode requires --harness flag. Example: tap-skill install --harness cursor,claude --scope user".into());
    };

    // Determine scope
    let scope = if let Some(s) = scope_arg {
        match s.as_str() {
            "user" => Scope::User,
            "project" => Scope::Project,
            _ => return Err(format!("Invalid scope '{}'. Must be 'user' or 'project'", s).into()),
        }
    } else if is_interactive {
        prompt_scope()?
    } else {
        return Err("Non-interactive mode requires --scope flag. Example: tap-skill install --harness cursor --scope user".into());
    };

    // Resolve targets
    let targets = resolve_install_targets(&harnesses, scope, path)?;

    if targets.is_empty() {
        return Err("No install targets resolved. Check your harness selection.".into());
    }

    // Install to each target
    for target in targets {
        install_to(&target)?;
        println!("ok: installed TAP skill -> {}/tap", target.display());
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum Scope {
    User,
    Project,
}

fn prompt_harnesses() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("Select harness(es) to install TAP skill:");
    println!("  1) Cursor");
    println!("  2) Claude (Claude Code / .claude)");
    println!("  3) Codex");
    println!("  4) Custom path (specify with --path)");
    println!();
    print!("Enter numbers (comma-separated, e.g., 1,2): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selections: Vec<&str> = input.trim().split(',').map(|s| s.trim()).collect();
    let mut harnesses = Vec::new();

    for sel in selections {
        match sel {
            "1" => harnesses.push("cursor".to_string()),
            "2" => harnesses.push("claude".to_string()),
            "3" => harnesses.push("codex".to_string()),
            "4" => harnesses.push("custom".to_string()),
            _ => {
                return Err(format!("Invalid selection: '{}'. Use 1, 2, 3, or 4", sel).into())
            }
        }
    }

    if harnesses.is_empty() {
        return Err("No harness selected. Aborting.".into());
    }

    Ok(harnesses)
}

fn prompt_scope() -> Result<Scope, Box<dyn std::error::Error>> {
    println!();
    println!("Select scope:");
    println!("  1) User (global: install to ~/.cursor/skills, etc.)");
    println!("  2) Project (local: install to .cursor/skills in current directory)");
    println!();
    print!("Enter number (1 or 2): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => Ok(Scope::User),
        "2" => Ok(Scope::Project),
        _ => Err(format!("Invalid selection: '{}'. Use 1 or 2", input.trim()).into()),
    }
}

fn resolve_install_targets(
    harnesses: &[String],
    scope: Scope,
    custom_path: Option<PathBuf>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut targets = Vec::new();

    let base_dir = match scope {
        Scope::User => std::env::var("HOME")
            .map(PathBuf::from)
            .map_err(|_| "HOME environment variable not set")?,
        Scope::Project => std::env::current_dir()?,
    };

    for harness in harnesses {
        match harness.as_str() {
            "cursor" => targets.push(base_dir.join(".cursor/skills")),
            "claude" => targets.push(base_dir.join(".claude/skills")),
            "codex" => targets.push(base_dir.join(".codex/skills")),
            "custom" => {
                if let Some(ref p) = custom_path {
                    targets.push(p.clone());
                } else {
                    return Err(
                        "Custom harness selected but no --path provided. Use --path <dir>".into(),
                    );
                }
            }
            _ => {
                return Err(format!(
                    "Unknown harness: '{}'. Use cursor, claude, codex, or custom",
                    harness
                )
                .into())
            }
        }
    }

    Ok(targets)
}


fn install_to(target: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let tap_target = target.join("tap");
    fs::create_dir_all(&tap_target)?;

    fn copy_dir_recursive(
        dir: &include_dir::Dir,
        dest: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for file in dir.files() {
            // Get just the file name (last component)
            let file_name = file
                .path()
                .file_name()
                .ok_or("invalid file path")?;
            let dest_file = dest.join(file_name);
            fs::write(&dest_file, file.contents())?;
        }
        for subdir in dir.dirs() {
            let dir_name = subdir
                .path()
                .file_name()
                .ok_or("invalid directory path")?;
            let dest_subdir = dest.join(dir_name);
            fs::create_dir_all(&dest_subdir)?;
            copy_dir_recursive(subdir, &dest_subdir)?;
        }
        Ok(())
    }

    let tap_dir =
        SKILLS_DIR
            .get_dir("tap")
            .ok_or("tap directory not found in embedded files")?;
    copy_dir_recursive(tap_dir, &tap_target)?;

    Ok(())
}

fn cmd_doctor() -> Result<(), Box<dyn std::error::Error>> {
    println!("TAP skill installation diagnostics\n");

    let home = match std::env::var("HOME").map(PathBuf::from) {
        Ok(h) => h,
        Err(_) => {
            println!("Error: HOME environment variable not set");
            return Ok(());
        }
    };

    let locations = vec![
        ("Cursor", home.join(".cursor/skills/tap")),
        ("Claude", home.join(".claude/skills/tap")),
        ("Codex", home.join(".codex/skills/tap")),
    ];

    for (name, path) in locations {
        let skill_file = path.join("SKILL.md");
        let exists = skill_file.exists();
        let status = if exists { "✓ installed" } else { "✗ not found" };
        println!("{:12} {} -> {}", name, status, path.display());
    }

    println!("\nTo install: tap-skill install [--cursor|--claude|--codex|--path <dir>]");
    Ok(())
}
