# gibo-wrapper

[`gibo`](https://github.com/simonwhitaker/gibo) is a great tool to create `.gitignore` files.
しかし，`.gitignore`を管理する，という観点においては改善の余地があるように思う．
そこで`gibo`に対し，Pull Request を行ったが，著者は，そのようは方向性への成長は考えていないようであった．
そこで，`gibo`のラッパーを作成し，`gibo`の機能を拡張する．

## Usage

1. install `gibo` command.
2. define `alias` in your `.bashrc` or `.zshrc` file.
  * ```bash
alias gibo='gibo-wrapper $@'
```
3. Use `gibo-wrapper` as `gibo`.


## Additional Commands

### `list-ignore` command

list boilerplates of `.gitignore` files in the current directory.

```bash
$ gibo list-ignore
macOS            Linux            Windows          Go
VisualStudioCode NetBeans
```

### Additional features in `dump` command

#### append mode

If any arguments of `dump` command start with `+`, `gibo` dumps the boilerplates in append mode.
In append mode, at first, `gibo` extracts the list of boilerplates from `.gitignore` file in the current directory.
Then, we add the arguments of `dump` command to the list by removing `+`.
Finally, `gibo` dumps boilerplates of the resultant name list.

```
$ gibo list-ignore
macOS            Linux            Go
$ gibo dump +windows > .gitignore
$ gibo list-ignore
macOS            Linux            Go               Windows
```

### remove mode

Also, if any arguments of `dump` command start with `-`, `gibo` dumps the boilerplates in remove mode.
In remove mode, `gibo` removes the given names from the boilerplates list.

```bash
$ gibo list-ignore
macOS            Linux            Go               Windows
$ gibo dump -windows > .gitignore
$ gibo list-ignore
macOS            Linux            Go
```

```bash
$ gibo dump macos linux windows -windows > .gitignore
$ gibo list-ignore
macOS            Linux            Go
```
