#![allow(unstable)]
extern crate "codius-sandbox-core" as sandbox;
extern crate "posix-ipc" as ipc;
use ipc::signals;

#[test]
fn exec_bin_false() {
    let argv = ["/usr/bin/false"];
    let exec = sandbox::executors::Execv::new(&argv);
    let mut sbox = sandbox::Sandbox::new(Box::new(exec));
    sbox.spawn();
    assert!(sbox.get_pid() != -1);
    loop {
        match sbox.tick().state {
            sandbox::events::State::Exit(st) => {
                assert!(st == 1);
                break;
            },
            _ => {}
        }
    }
}

#[test]
fn exec_bin_true() {
    let argv = ["/usr/bin/true"];
    let exec = sandbox::executors::Execv::new(&argv);
    let mut sbox = sandbox::Sandbox::new(Box::new(exec));
    sbox.spawn();
    assert!(sbox.get_pid() != -1);
    loop {
        match sbox.tick().state {
            sandbox::events::State::Exit(st) => {
                assert!(st == 0);
                break;
            },
            _ => {}
        }
    }
}

#[test]
fn exec_closure() {
    let exec = sandbox::executors::Function::new(Box::new(move |&:| -> i32 {0}));
    let mut sbox = sandbox::Sandbox::new(Box::new(exec));
    sbox.spawn();
    assert!(sbox.get_pid() != -1);
    loop {
        match sbox.tick().state {
            sandbox::events::State::Exit(st) => {
                assert!(st == 0);
                break;
            },
            _ => {}
        }
    }
}

#[test]
fn exec_closure_with_return() {
    let exec = sandbox::executors::Function::new(Box::new(move |&:| -> i32 {42}));
    let mut sbox = sandbox::Sandbox::new(Box::new(exec));
    sbox.spawn();
    assert!(sbox.get_pid() != -1);
    loop {
        match sbox.tick().state {
            sandbox::events::State::Exit(st) => {
                assert!(st == 42);
                break;
            },
            _ => {}
        }
    }
}

#[test]
fn release() {
    let exec = sandbox::executors::Function::new(Box::new(move |&:| -> i32 {0}));
    let mut sbox = sandbox::Sandbox::new(Box::new(exec));
    sbox.spawn();
    assert!(sbox.get_pid() != -1);
    sbox.release(ipc::signals::Signal::Cont);
    assert!(sbox.get_pid() == -1);
}
