# gibo-wrapper

[![build](https://github.com/tamada/gibo-wrapper/actions/workflows/build.yaml/badge.svg)](https://github.com/tamada/gibo-wrapper/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/tamada/gibo-wrapper/badge.svg?branch=main)](https://coveralls.io/github/tamada/gibo-wrapper?branch=main)
[![Go Report Card](https://goreportcard.com/badge/github.com/tamada/gibo-wrapper)](https://goreportcard.com/report/github.com/tamada/gibo-wrapper)

[![gibo-wrapper](https://img.shields.io/badge/gibo--wrapper-v0.5.8-blue)](https://github.com/tamada/gibo-wrapper/releases/tag/v0.5.8)
[![Unlicense license](http://img.shields.io/badge/license-Unlicense-blue.svg?style=flat)](LICENSE)

[`gibo`](https://github.com/simonwhitaker/gibo) is a great tool for creating `.gitignore` files.
However, `gibo` can improve some features in the aspect of managing `.gitignore`.
Then, I sent [a pull request](https://github.com/simonwhitaker/gibo/pull/61) to `gibo`. However, the author does not think for growing in the direction.
Therefore, I created the wrapper of `gibo` to extend the features.

## Usage

1. install `gibo` command.
2. define `alias` in your `.bashrc` or `.zshrc` file.
  * `alias gibo='gibo-wrapper $@'`
3. Use `gibo-wrapper` as `gibo`.

## Install

### :beer: Homebrew

```sh
brew install tamada/tap/gibo-wrapper
```

### :muscle: Compile yourself

```sh
git clone https://github.com/tamada/gibo-wrapper
cd gibo-wrapper
make
# put the resultant executable 'gibo-wrapper' into the suitable location.
```

## Additional Commands

In the following example, `gibo` command is actually `gibo-wrapper`, it is aliased.

### `list-ignore` command

list boilerplates of `.gitignore` files in the current directory.

```bash
$ gibo list-ignore
macOS            Linux            Windows          Go
VisualStudioCode NetBeans
```

### Additional features in `dump` command

#### append mode

If any arguments of the `dump` command start with `+`, `gibo` dumps the boilerplates in append mode.
In append mode, at first, `gibo` extracts the list of boilerplates from the `.gitignore` file in the current directory.
Then, we add the arguments of the `dump` command to the list by removing `+`.
Finally, `gibo` dumps boilerplates of the resultant name list.

```
$ gibo list-ignore
macOS            Linux            Go
$ gibo dump +windows > .gitignore
$ gibo list-ignore
macOS            Linux            Go               Windows
```

#### remove mode

Also, if any arguments of the `dump` command start with `_`, `gibo` dumps the boilerplates in remove mode.
In the remove mode, `gibo` removes the given names from the boilerplates list.

```bash
$ gibo list-ignore
macOS            Linux            Go               Windows
$ gibo dump _windows > .gitignore
$ gibo list-ignore
macOS            Linux            Go
```

```bash
$ gibo dump macos linux windows _windows > .gitignore
$ gibo list-ignore
macOS            Linux            Go
```

#### `--keep-prologue` option

In the use of `gibo`, we sometimes add some content to the top of the `.gitignore` file.
In such a case, we want to keep the content.
`--keep-prologue` option is for this purpose.

#### `--remove-duplication` option

In the use of `gibo`, adding/removing boilerplates may cause the duplication of boilerplates.
`--remove-duplication` option removes duplicated boilerplates and dumps them.

## `init` command

`init` command is for initializing `gibo` and `gibo-wrapper` commands in the shell configuration file.
Add the following line in your shell configuration file, such as `.zshrc` and `.bashrc`.

```sh
eval "$(gibo-wrapper init $SHELL)"
```
