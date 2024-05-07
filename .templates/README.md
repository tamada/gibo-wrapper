# gibo-wrapper

[![build](https://github.com/tamada/gibo-wrapper/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/gibo-wrapper/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/gibo-wrapper/badge.svg?branch=main)](https://coveralls.io/github/tamada/gibo-wrapper?branch=main)

[![gibo-wrapper](https://img.shields.io/badge/gibo--wrapper-v${VERSION}-blue)](https://github.com/tamada/gibo-wrapper/releases/tag/v${VERSION})
[![Unlicense license](http://img.shields.io/badge/license-Unlicense-blue.svg?style=flat)](LICENSE)

[`gibo`](https://github.com/simonwhitaker/gibo) is a great tool for creating `.gitignore` files.
However, `gibo` can improve some features in the aspect of managing `.gitignore`.
Then, I sent [a pull request](https://github.com/simonwhitaker/gibo/pull/61) to `gibo`. However, it was not accepted because it seemed to be different from the author's direction.
Therefore, I created the wrapper of `gibo` to introduce the new features for managing `.gitignore`.

## Install

### :beer: Homebrew

```sh
brew install tamada/tap/gibo-wrapper
```

### :muscle: Compile yourself

```sh
git clone https://github.com/tamada/gibo-wrapper
cd gibo-wrapper
cargo build --release
# put the resultant executable 'gibo-wrapper' into the suitable location.
```

### Setup `gibo-wrapper`

1. install `gibo` command.
2. define `alias` in your `.bashrc` or `.zshrc` file.
  * `alias gibo='gibo-wrapper $@'`
3. Use `gibo-wrapper` as `gibo`.

## :runner: Usage

`gibo-wrapper` introduces the new commands `current-list` and update `dump` command for applying some options.

### `gibo-wrapper current-list`

`current-list` sub-command for `gibo-wrapper` shows the list of boilerplates in the `.gitignore` file in the current directory.

```sh
List the current boilerplates in the .gitignore file

Usage: gibo-wrapper current-list

Options:
  -h, --help  Print help
```

#### Example

```bash
$ gibo current-list
macOS            Linux            Windows          Go
VisualStudioCode JetBrains
```

### `gibo-wrapper dump`

The `gibo-wrapper` updates `dump` sub-command for adding four new options, `--keep-prologue`, `--remove-duplication`, `--in-place`, and `--verbose`, and two new modes, append mode and remove mode.

```sh
Dump a boilerplate

Usage: gibo-wrapper dump [OPTIONS] [ARGS]...

Arguments:
  [ARGS]...  the boilerplate names to dump.
             Append boilerplates into the current .gitignore file if the name starts with `+`.
             Remove boilerplates from the current .gitignore file if the name starts with `_`.

Options:
  -k, --keep-prologue       Keep the prologue of the .gitignore
  -r, --remove-duplication  Remove the duplicated boilerplate names
  -i, --in-place            Update .gitignore files in-place
  -v, --verbose             Show verbose output
  -h, --help                Print help
```

In the following example, `gibo` is `gibo-wrapper`, it is aliased.

#### append mode

If the arguments of the `dump` command start with `+`, `gibo` dumps the boilerplates in append mode.
In append mode, at first, `gibo` extracts the list of boilerplates from the `.gitignore` file in the current directory.
Then, we add the arguments of the `dump` command to the list by removing `+`.
Finally, `gibo` dumps boilerplates of the resultant name list.

```
$ gibo current-list
macOS            Linux            Go
$ gibo dump --in-place +windows 
$ gibo current-list
macOS            Linux            Go               Windows
```

#### remove mode

Also, if any arguments of the `dump` command start with `_`, `gibo` dumps the boilerplates in remove mode.
In the remove mode, `gibo` removes the given names from the boilerplates list.

```bash
$ gibo current-list
macOS            Linux            Go               Windows
$ gibo dump --in-place _windows
$ gibo current-list
macOS            Linux            Go
```

```bash
$ gibo dump --in-place macos linux windows _windows
$ gibo current-list
macOS            Linux            Go
```

#### `--in-place` option

`--in-place` option is for updating the `.gitignore` file in the current directory.
If you use redirect (`>`), such as `gibo dump +macos > .gitignore`, at first `.gitignore` file is truncated and then the `gibo` is executed.
Therefore, append, and remove mode cannot be used with redirect.
Hence, `gibo-wrapper` introduces `--in-place` option for updating `.gitignore`.

#### `--keep-prologue` option

In the use of `gibo`, we sometimes add some content to the top of `.gitignore` file.
In such a case, we want to keep the content.
`--keep-prologue` option is for this purpose.

#### `--remove-duplication` option

In the use of `gibo`, adding/removing boilerplates may cause the duplication of boilerplates.
`--remove-duplication` option removes duplicated boilerplates and dumps them.
