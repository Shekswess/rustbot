// Changed in the cppalliance fork of RustBot: tests of the option --now. See "Changes in this fork" in README.md.

use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

fn run(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustbot"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start rustbot");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().expect("wait for rustbot")
}

#[test]
fn seeded_pipe_preserves_blank_turns_and_accepts_crlf() {
    let prompts = "My name is Sam.\n\nWhat's my name?\nWhat is the capital of Italy?\n";
    let args = ["--seed=0", "-v"];
    let lf = run(&args, prompts);
    let crlf = run(&args, &prompts.replace('\n', "\r\n"));
    assert!(lf.status.success());
    assert!(crlf.status.success());
    assert_eq!(lf.stdout, crlf.stdout);
    let reply = String::from_utf8(lf.stdout).unwrap();
    assert_eq!(reply.lines().count(), 4);
    assert!(reply.contains("Sam"));
    assert!(reply.ends_with("Rome.\n"));
    assert!(
        String::from_utf8(lf.stderr)
            .unwrap()
            .lines()
            .all(|line| line.starts_with("[module: "))
    );
}

#[test]
fn an_interactive_reset_clears_previous_conversation_state() {
    let dialogue = run(
        &["--seed=0", "-i", "-p", "My name is Sam."],
        "What's my name?\n\n/HeLp\n /ReSeT \nWhat's my name?\n/quit\nignored\n",
    );
    assert!(dialogue.status.success());
    let reply = String::from_utf8(dialogue.stdout).unwrap();
    let (before, after) = reply.split_once("New conversation.").unwrap();
    assert!(
        before.contains("rustbot: ")
            && before.lines().filter(|line| line.contains("Sam")).count() >= 3
    );
    assert!(
        after.contains("haven't told me your name") || after.contains("don't know it yet"),
        "{after}"
    );
    assert!(!after.contains("Sam") && !after.contains("ignored"));
}

#[test]
fn the_now_option_sets_the_date_that_a_reply_reads() {
    let ask = "When is the next full moon?";
    let november = run(
        &[
            "--seed=0",
            "--now",
            "2026-11-22T00:00:00+00:00[UTC]",
            "-p",
            ask,
        ],
        "",
    );
    assert!(november.status.success());
    let reply = String::from_utf8(november.stdout).unwrap();
    assert!(
        reply.starts_with("The next full moon is on Tuesday 24 November, in 2 days."),
        "{reply}"
    );

    // One reply per piped line, each at the same set time.
    let piped = run(
        &["--seed=0", "--now=2031-03-05T09:30:00+00:00[UTC]"],
        &format!("{ask}\n{ask}\n"),
    );
    assert!(piped.status.success());
    let replies = String::from_utf8(piped.stdout).unwrap();
    assert_eq!(replies.lines().count(), 2);
    assert!(
        replies
            .lines()
            .all(|line| line.starts_with("The next full moon is on Saturday 8 March, in 3 days.")),
        "{replies}"
    );
}

#[test]
fn a_now_option_that_cannot_be_read_ends_with_an_error() {
    let bad = run(&["--seed=0", "--now", "tomorrow", "-p", "Hello"], "");
    assert!(!bad.status.success());
    assert!(bad.stdout.is_empty());
    let error = String::from_utf8(bad.stderr).unwrap();
    assert!(error.contains("cannot read the time"), "{error}");
}

#[test]
fn one_shot_reads_the_prompt_without_consuming_stdin() {
    let reply = run(
        &["--seed=0", "-v", "-p", "What is the capital of Italy?"],
        "Tell me about France\n",
    );
    assert!(reply.status.success());
    assert_eq!(reply.stdout, b"Rome.\n");
    assert!(
        String::from_utf8(reply.stderr)
            .unwrap()
            .starts_with("[module: answers | ")
    );
}
