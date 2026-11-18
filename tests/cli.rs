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
