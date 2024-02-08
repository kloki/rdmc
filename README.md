# RDMC - Readme command

Run commands from you readme as if its a Makefile.

## Installation

```bash
cargo install rdmc
```

### Setup Autocomplete Bash

```bash
rdmc --bpaf-complete-style-bash >> ~/.bash_completion
```

# Usage

When you invoke `rdmc` it will look for a readme file in your current directory. It will extract commands from it.

```bash
rdmc <Tab>
rdmc C<Tab>
rdmc Command
```

## Execute

`rdmc` will only printout the actual command. To run it you can pipe it to a shell.

```bash
rdmc <Command> | sh
```

# Readme format

`rdmc` will extract commands from all code blocks tagged with the `bash` language. The name of the command is based on its closest heading.

Reference this README file as a properly formatted example.
