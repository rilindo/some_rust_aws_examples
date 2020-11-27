# Some Rust AWS Examples.

This here contains code for accessing AWS API using Rust.

## Overview

I wanted to practice my new found knowledge [rust](https://www.rust-lang.org/)   with something practical, which, in this, using the AWS API. As turns out, as there wasn't much by way of examples out at the internet, which, given my relative nascent knowledge of the language, proved.

But I persisted and eventually, I managed to figured out how to write some basic code. After a few weeks, I got to the point where I can execute any AWS operation with relative ease. So I was fairly happy.

That said, it bothers me that it there isn't much in way for any new rust developer to access AWS services easily searchable on the internet. So that nobody have to go through what I have went through, I am writing small rust code that access or execute a specific action with AWS. Hopefully, at some point, this site will start showing up in the searches and maybe. . . it makes life a little easier for somebody.

But if not, at least I can get to practice and advance my rust skills. :)rience of trying to use Rust with AWS.

## What This Repo Contains

The repo is divided to directories that maps to a specific AWS service. Each directory, in turn, contains sample code to either perform a list/describe operation, a create operations or (in a few cases, a delete operation). Most of the example uses [rusoto](https://www.rusoto.org/) for executing AWS APIs, with some using [clap](https://docs.rs/clap/2.33.3/clap/) for passing parameters.

## How to Use This Repo

You can build most of the examples by simply running `cargo build` at the root of the project. To run an example command, you can simply:

`cargo run <command_name>`

For commands that requires paramaters, you will need to insert `--` between the `run` subcommand and the parameter:

`cargo run <command_name> -- --param_name value`

You are free to examine code and use it as a starting point for your code. Note that it is your responsibility to make it production ready if you so choose so in your day job. :)

## How to Contribute

If you find this useful and even have some things to contribute, you are free to create a PR to add any example code of your own or corrections if necessary. If you do have questions about the code, open a ticket/issue  and I'll see if I can answer them.

## TBD

- Basic Rusoto tutorial.
- Unit Tests. I need to add unit tests.
- Multi-operation example code.
