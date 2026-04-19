<div align="center">
  <h1><code>crossterm-simple-event</code></h1>

  <p>
    <strong>Turn crossterm events into easy to match strings</strong>
  </p>

  <p>
    <a href="https://github.com/wyvernbw/crossterm-simple-event/actions"><img src="https://img.shields.io/github/actions/workflow/status/wyvernbw/crossterm-simple-event/ci.yaml" alt="GitHub Actions" /></a>
    <a href="https://docs.rs/crossterm-simple-event"><img src="https://img.shields.io/docsrs/crossterm-simple-event" alt="docs.rs" /></a>
    <a href="https://crates.io/crates/crossterm-simple-event"><img src="https://img.shields.io/crates/v/crossterm-simple-event" alt="crates.io" /></a>
    <a href="https://github.com/wyvernbw/crossterm-simple-event/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/crossterm-simple-event" alt="License" /></a>
  </p>
</div>

<br>

Turn complex `crossterm::event::Event` into nice keybind strings like:

```text
ctrl+c
ctrl+shift+d
alt+enter
f5
super+s
shift+voldown
```

## Getting started

Import the extension trait

```rust
use crossterm_simple_event::CrosstermSimpleEvent;
```

Call `simple` on a `crossterm::event::Event` and match on the value.

```rust
// read event from crossterm
let ev = crossterm::event::read().unwrap();
// call `simple` and turn into &str
match ev.simple().as_str() {
    "ctrl+c" => {
        print("done.");
        break;
    }
    "b" => {
        print("hello world!");
    }
    "p" => {
        panic!("at the disco");
    }
    _ => print("naughty naughty"),
}
```

## AI Policy

This repo, small as it is, does **not** contain or accept LLM generated code.
Remember to never offload your creativity to the machine.
