# Contributing to Ruffle

🎉 Thanks for your interest in Ruffle! Contributions of all kinds are welcome.

This document serves as a general guide for contributing to Ruffle. Follow your best judgement in following these guidelines.

## Table of Contents

* [Getting Started](#getting-started)
* [Ways to Contribute](#ways-to-contribute)
    * [Test your favorite Flash content](#test-your-favorite-flash-content)
    * [Improve documentation](#improve-documentation)
    * [Fix interesting issues](#fix-interesting-issues)
    * [Implement missing Flash functionality](#implement-missing-flash-functionality)
* [Debugging ActionScript Content](#debugging-actionscript-content)
* [Reporting Bugs](#reporting-bugs)
* [Code Guidelines](#code-guidelines)
* [Test Guidelines](#test-guidelines)
* [Commit Message Guidelines](#commit-message-guidelines)
* [Pull Requests](#pull-requests)

## Getting Started

The [Ruffle wiki](https://github.com/ruffle-rs/ruffle/wiki) is a great way to familiarize yourself with the project. It contains info on how to build Ruffle, using Ruffle, and links to helpful documentation about the Flash format.

Feel free to ask questions in our [Discord server](https://discord.gg/ruffle).

## Reverse Engineering requirements

Ruffle does not use any proprietary knowledge or code, and is built entirely upon either inspecting the output of Flash Player, or by consulting license-compatible libraries such as [avmplus](https://github.com/adobe/avmplus).

It is strictly forbidden to decompile Flash Player, Flash Professional, Adobe Animate, or any other software that does not explicitly permit doing so. Any contributions to Ruffle must be re-licensable to MIT/Apache and obtained through legitimate methods.

If you're unsure if something is allowed, ask in our [Discord server](https://discord.gg/ruffle)! The rule of thumb though is that if you made it, and you didn't decompile anything to get there, it's probably fine!

## Ways to Contribute

We love new contributors! You can contribute to Ruffle in several ways:

### Test your favorite Flash content

Try out your favorite SWF content in Ruffle and see how it works! Follow the instructions on the [Using Ruffle](https://github.com/ruffle-rs/ruffle/wiki/Using-Ruffle) page to get Ruffle for your desired platform. You can run the SWF through the desktop player, the web demo, or try the extension on live websites.

If you encounter specific issues with content, please follow the guidelines on filing an issue.

### Improve documentation

Improving documentation is a great way to learn the codebase. Adding documentation to both the wiki and the code eases the learning curve for both end users and new contributors.

For documentation in the code, we follow the [rustdoc](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments) guidelines.

### Fix interesting issues

Try your hand at fixing [issues that are interesting to you](https://github.com/ruffle-rs/ruffle/issues). Follow the instructions on [building Ruffle](https://github.com/ruffle-rs/ruffle/wiki/Building-Ruffle), familiarize yourself with the [project layout](https://github.com/ruffle-rs/ruffle/wiki/Project-Layout), and use [SWF resources and decompilers](https://github.com/ruffle-rs/ruffle/wiki/Helpful-Resources) to help debug the issue.

You can also ask for mentoring on our [Discord server](https://discord.gg/ruffle).

### Implement missing Flash functionality

Ruffle is a young project, and there is still much Flash functionality that is unimplemented. Check for the ["unimplemented"](https://github.com/ruffle-rs/ruffle/issues?q=is%3Aissue+is%3Aopen+label%3Aunimplemented) in issues.

## Debugging ActionScript Content

To enable debug logging, set `RUST_LOG=warn,ruffle_core=debug,avm_trace=trace` and run Ruffle from the command line. 
This will also enable printing `trace()` statements.

Additionally, if you build Ruffle with `--features avm_debug` then you will activate a few more built-in debugging utilities inside Ruffle, listed below.

### Warnings and Errors

All AVM errors and warnings will print their stack trace so that you can view where they are in relation to the
ActionScript inside the movie.

### Step-By-Step Output

The hotkey <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>D</kbd> toggles verbose AVM debugging output on and off (default off).
You will be able to follow the flow of ActionScript inside of a SWF movie, as each action is performed.
Please note that this will likely slow down Ruffle, and it may significantly spam output. Please use sparingly.

When paired with a tool such as [JPEXS](https://github.com/jindrapetrik/jpexs-decompiler), you can compare the ActionScript
you see being executed in Ruffle with the actual ActionScript inside of a SWF movie, and attempt to find whatever problem
it is that you're looking for.

### Complete Variable Dumping

The hotkey <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>V</kbd> dumps every variable inside the AVM at the moment you press it.
This can be very useful to inspect the internal state of games and see, for example, if a coordinate is NaN, your lives
are negative, or maybe an important object just didn't get initialized.

This currently only works for AVM1. We'd [welcome a PR to change that](https://github.com/ruffle-rs/ruffle/issues/8951)!

### Render Tree Dumping

The hotkey <kbd>Ctrl</kbd>+<kbd>Alt</kbd>+<kbd>F</kbd> dumps the DisplayObject render tree at the moment you press it. 
This allows you to see Ruffle's representation of the objects on the Stage.

## Reporting Bugs

[Issue reports and feature requests](https://github.com/ruffle-rs/ruffle/issues) are encouraged, and are a great way to measure our progress!

When filing an issue, if possible, please include:

* A clear description of the problem.
* The platform you are testing on (web, desktop, OS).
* A link/attachment to the SWF demonstrating the issue, if possible.
* Screenshots if the issue is a visible problem.
* Bonus points for including the correct output from the official Flash Player.

These types of focused issues are helpful:

* Tracking issues for specific Flash features (ActionScript 3.0, drawing API, etc.)
* Bug reports for specific content that works but isn't quite right (art not looking correct, etc.)
* Platform-specific issues
* Enhancement requests to improve user experience

The project is still in the early stages, so many Flash features are unimplemented and not yet expected to work. Please avoid filing generic issues such as:

* A "this SWF doesn't work at all" report (what about it doesn't work?).
* Duplicate issues for each piece of content using an unimplemented feature.
* Asking for dates when a feature will be implemented.

## Code Guidelines

Ruffle is built using the latest stable version of the Rust compiler. Nightly and unstable features should be avoided.

The Rust code in Ruffle strives to be idiomatic. The Rust compiler should emit no warnings when building the project. Additionally, all code should be formatted using [`rustfmt`](https://github.com/rust-lang/rustfmt) and linted using [`clippy`](https://github.com/rust-lang/rust-clippy). You can install these tools using `rustup`:

```sh
rustup component add rustfmt
rustup component add clippy
```

You can auto-format your changes with `rustfmt`:

```sh
cargo fmt --all
```

and you can run the clippy lints:

```sh
cargo clippy --all --tests
```

Specific warnings and clippy lints can be allowed when appropriate using attributes, such as:

```rs
#[allow(clippy::float_cmp)]
```

## Test Guidelines

Heavily algorithmic code may benefit from unit tests in Rust: create a module `mod tests` conditionally compiled with `#[cfg(test)]`, and add your tests in there.

Most tests are SWF-based, with the SWFs stored in `tests/tests/swfs/`. They are configured in `tests/tests/regression_tests.rs`. To add a new test, create an `.swf` that runs `trace()` statements. You can do this in several ways, listed below.

Once you have an `.swf`, run it in Flash Player and create a file `output.txt` with the contents of the trace statements. Add the `output.txt`, `test.swf` and either the `test.as` or `test.fla` file to a directory under `tests/tests/swfs/avm1` (or `avm2`) named after what your test tests.

Finally, add a `test.toml` in the same directory to control how the test is run - such as how many frames it should take or if we should compare the image it generates. See [tests/README.md](tests/README.md) for information on how the test.toml should look like.

Running `cargo test [your test]` will run the `.swf` in Ruffle and compare the `trace()` output against `output.txt`.

### Flash authoring tool

Create a new ActionScript project. Save the `.fla` file and export an `.swf` (File -> Export -> Export Movie...).

Adobe Flash Professional CS6 is the most recent version to support both ActionScript 2 and 3. Newer versions support ActionScript 3 only.

### Motion-Twin ActionScript 2 Compiler

This is a free and open source command-line ActionScript 2 compiler. It can be downloaded from [here](http://tech.motion-twin.com/mtasc.html#download).

Create a `test.as` file in a text editor, per the following template:

```as
class Test {
    static function main() {
        // Your test here.
        trace("Hello World!");
    }
}
```

Then compile it using:

```sh
mtasc -main -header 200:150:30 test.as -swf test.swf
```

### Apache Flex SDK

This is a free and open source SDK capable of compiling ActionScript 3 code. It can be set up as follows:

1. Download a release for your platform from [here](https://flex.apache.org/download-binaries.html), and extract the files somewhere.
2. Add the `<sdk-root>/bin` directory to your `PATH`. After that, the command-line compiler `mxmlc` (among other tools) should be available.
3. `mxmlc` needs a `playerglobal.swc` in order to work, which can be grabbed from [here](https://fpdownload.macromedia.com/get/flashplayer/updaters/32/playerglobal32_0.swc). Place it in `<sdk-root>/frameworks/libs/player/32.0/playerglobal.swc`, while creating intermediate `player` and `32.0` directories.
4. Define the `FLEX_HOME` and `PLAYERGLOBAL_HOME` environment variables to the path of the extracted SDK root, and the path of the `<sdk-root>/frameworks/libs/player` subdirectory, respectively.
5. Edit `<sdk-root>/frameworks/flex-config.xml` and change `<target-player>27.0</target-player>` to `<target-player>32.0</target-player>`.

After `mxmlc` is set up, create a file `test.as` in a text editor, per the following template:

```as
package {
    public class Test {}
}

// Your test here.
trace("Hello World!");
```

Then compile it using:

```sh
mxmlc -output test.swf -compiler.debug=true Test.as
```

You may want to use Docker instead - something like `docker run -it --rm -v ${PWD}:/src jeko/airbuild mxmlc -output test.swf -compiler.debug=true Test.as` works well.

### RABCDAsm

[RABCDAsm](https://github.com/CyberShadow/RABCDAsm) allows writing AVM2 bytecode sequences directly, without intermediate AS3 code, which is primarily useful for testing opcodes that aren't generated by the above-mentioned AS3 compilers.
However it cannot generate SWF files from scratch. Instead, you must first generate a SWF from the above mentioned methods, then extract and disassemble its ABC with `abcexport` and `rabcdasm`.
Once you have modified your bytecode, you must reassemble and inject it into the movie with `rabcasm` and `abcreplace`.
If you are adding a new test, commit both your SWF source (`.fla` and/or `.as` files) as well as the modified bytecode (`.abc` files and `test-0` folder).

## Commit Message Guidelines

Here is a sample commit message:

```
web: Fix incorrect rendering of gradients (close #23)
```

* If applicable, prefix the first line with a tag indicating the relevant area of changes:
    * `core:`
    * `desktop:`
    * `web:`
    * `avm1:`
    * `avm2:`
    * `docs:`
    * `chore:`
    * `tests:`
* Capitalize the first letter following the tag.
* Limit line length to 72 characters.
* Use the present tense and imperative mood ("fix", not "fixed" nor "fixes").
* Reference any PRs or issues in the first line.
* Use keywords to close/address issues when applicable ("close #23").
* Write more detailed info on following lines when applicable.

## Pull Requests

Pull requests are the primary way to contribute code to Ruffle. Pull requests should be made against the latest `master` branch. Your pull request should not contain merges; you should always rebase when bringing the latest changes into your branch from the `master` branch. If there are merge conflicts, or if your commit history is messy, please rebase onto the latest master. [`git rebase -i`](https://thoughtbot.com/blog/git-interactive-rebase-squash-amend-rewriting-history#interactive-rebase) is a great way to clean up your pull request.

When you make a pull request, our [CI](https://github.com/ruffle-rs/ruffle/actions) will build your changes and run them through all tests and style checks. All of these tests should pass before your pull request can be accepted.

One of [our regular contributors](https://github.com/orgs/ruffle-rs/people) will review your changes and try their best to helpfully suggest any changes. If all goes well, your PR should be merged without much delay. We use both standard merge commits and fast-forward merges depending on the size of the changes. Thanks for your contribution!
