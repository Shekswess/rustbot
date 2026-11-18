use clap::Parser;
use rustbot::{Bot, SOURCE_COMMIT, one_line};
use rustyline::{DefaultEditor, error::ReadlineError};
use std::{
    error::Error,
    io::{self, BufRead, IsTerminal, Write},
    process::ExitCode,
    time::Instant,
};

const LOGO: &str = r"    ____             __  ____        __
   / __ \__  _______/ /_/ __ )____  / /_
  / /_/ / / / / ___/ __/ __  / __ \/ __/
 / _, _/ /_/ (__  ) /_/ /_/ / /_/ / /_
/_/ |_|\__,_/____/\__/_____/\____/\__/";
const TAGLINE: &str =
    "A rule-based chatbot with a Rust engine. No weights, no training, just rules.";
const HINT: &str = "Type a message and press Enter. /reset starts a new conversation, /exit quits.";
const COMMANDS: &str = "Commands:
  /reset   clear the screen and start again (RustBot forgets what was said)
  /help    show this help
  /exit    quit (Ctrl+D works too)";

#[derive(Parser)]
#[command(
    disable_version_flag = true,
    about = "Chat with RustBot. No model, no weights.",
    after_help = "With no options, RustBot starts an interactive chat. When input is piped in, it prints one reply per input line."
)]
struct Args {
    /// Reply to one message and exit.
    #[arg(short, long, value_name = "TEXT")]
    prompt: Option<String>,
    /// With -p, keep chatting after the first reply.
    #[arg(short, long)]
    interactive: bool,
    /// Seed the choice of wording.
    #[arg(short, long, allow_negative_numbers = true, value_name = "N", value_parser = parse_seed)]
    seed: Option<String>,
    /// Show which module produced the reply and how long it took.
    #[arg(short, long)]
    verbose: bool,
    /// Show the binary version and the original rule revision.
    #[arg(long)]
    version: bool,
    /// Emit one JSON turn record per input line for parity checks.
    #[arg(long, conflicts_with_all = ["prompt", "interactive", "inventory", "verbose"])]
    trace: bool,
    /// Emit the normalized fields and classification for each input line.
    #[arg(long, conflicts_with_all = ["prompt", "interactive", "inventory", "verbose", "trace"])]
    read: bool,
    /// List loaded modules and knowledge domains as JSON.
    #[arg(long, conflicts_with_all = ["prompt", "interactive", "verbose"])]
    inventory: bool,
    /// List registered domain and card IDs in matcher order for the migration verifier.
    #[arg(long, conflicts_with_all = ["prompt", "interactive", "verbose", "inventory", "trace", "read"])]
    knowledge_ids: bool,
}

fn parse_seed(seed: &str) -> Result<String, String> {
    let digits = seed.strip_prefix(['+', '-']).unwrap_or(seed);
    let mut parts = digits.split('_');
    if parts.all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit())) {
        Ok(seed.to_owned())
    } else {
        Err("expected a decimal integer".into())
    }
}

fn read_line(input: &mut impl BufRead, line: &mut String) -> io::Result<bool> {
    line.clear();
    if input.read_line(line)? == 0 {
        return Ok(false);
    }
    line.truncate(line.trim_end_matches(['\r', '\n']).len());
    Ok(true)
}

fn details(module: Option<&str>, start: Instant) -> String {
    format!(
        "[module: {} | {:.0} ms]",
        module.unwrap_or("none"),
        start.elapsed().as_secs_f64() * 1000.0
    )
}

fn opening(output: &mut impl Write) -> io::Result<()> {
    writeln!(output, "{LOGO}\n\n{TAGLINE}\n{HINT}\n")?;
    output.flush()
}

fn chat(
    bot: &mut Bot,
    args: &Args,
    input: &io::Stdin,
    output: &mut impl Write,
    tty_in: bool,
    tty_out: bool,
) -> Result<(), Box<dyn Error>> {
    let mut editor = if tty_in && tty_out {
        Some(DefaultEditor::new()?)
    } else {
        None
    };
    opening(output)?;
    let mut first = args.prompt.as_deref();
    let mut buffer = String::new();
    loop {
        let text = if let Some(text) = first.take() {
            writeln!(output, "user: {text}")?;
            text
        } else if let Some(editor) = &mut editor {
            match editor.readline("user: ") {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        editor.add_history_entry(&line)?;
                    }
                    buffer = line;
                    &buffer
                }
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    writeln!(output)?;
                    break;
                }
                Err(error) => return Err(error.into()),
            }
        } else {
            write!(output, "user: ")?;
            output.flush()?;
            if !read_line(&mut input.lock(), &mut buffer)? {
                writeln!(output)?;
                break;
            }
            &buffer
        };
        match text.trim().to_lowercase().as_str() {
            "/exit" | "/quit" | "/bye" => break,
            "/reset" => {
                bot.reset()?;
                if tty_out {
                    write!(output, "\x1b[H\x1b[2J\x1b[3J")?;
                    opening(output)?;
                } else {
                    writeln!(output, "New conversation.\n")?;
                }
            }
            "/help" => writeln!(output, "{COMMANDS}\n")?,
            "" => {}
            _ => {
                let start = Instant::now();
                let turn = bot.turn(text);
                writeln!(output, "rustbot: {}", one_line(&turn.reply))?;
                if args.verbose {
                    writeln!(output, "{}", details(turn.module, start))?;
                }
                writeln!(output)?;
            }
        }
        output.flush()?;
    }
    Ok(())
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    if args.version {
        println!(
            "rustbot {} (bot at commit {})",
            env!("CARGO_PKG_VERSION"),
            &SOURCE_COMMIT[..12]
        );
        return Ok(());
    }
    let stdin = io::stdin();
    let stdout = io::stdout();
    if args.knowledge_ids {
        let mut out = stdout.lock();
        serde::ser::Serializer::collect_seq(
            &mut serde_json::Serializer::new(&mut out),
            rustbot::knowledge_ids(),
        )?;
        writeln!(out)?;
        return Ok(());
    }
    let tty_in = stdin.is_terminal();
    let tty_out = stdout.is_terminal();
    let interactive = !args.inventory
        && !args.trace
        && !args.read
        && ((args.prompt.is_none() && tty_in) || (args.prompt.is_some() && args.interactive));

    if interactive {
        write!(io::stderr(), "Loading RustBot...")?;
        io::stderr().flush()?;
    }
    let mut bot = Bot::new(args.seed.as_deref())?;
    if args.inventory {
        let mut output = stdout.lock();
        serde_json::to_writer(&mut output, &bot.inventory()?)?;
        writeln!(output)?;
    } else if interactive {
        write!(io::stderr(), "\r                  \r")?;
        // Rustyline owns the terminal and acquires both streams as needed.
        chat(&mut bot, &args, &stdin, &mut io::stdout(), tty_in, tty_out)?;
    } else {
        let mut output = stdout.lock();
        let mut answer = |text: &str| -> Result<(), Box<dyn Error>> {
            if args.read {
                serde_json::to_writer(&mut output, &rustbot::inspect_line(text))?;
                writeln!(output)?;
                output.flush()?;
                return Ok(());
            }
            let start = Instant::now();
            let turn = bot.turn(text);
            if args.trace {
                serde_json::to_writer(&mut output, &turn)?;
                writeln!(output)?;
            } else {
                writeln!(output, "{}", one_line(&turn.reply))?;
            }
            output.flush()?;
            if args.verbose {
                eprintln!("{}", details(turn.module, start));
            }
            Ok(())
        };
        if let Some(prompt) = &args.prompt {
            answer(prompt)?;
        } else {
            let mut input = stdin.lock();
            let mut line = String::new();
            while read_line(&mut input, &mut line)? {
                answer(&line)?;
            }
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("rustbot: {error}");
            ExitCode::FAILURE
        }
    }
}
