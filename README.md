zsh-dirhist
===========

**This is currently a fork of https://github.com/tymm/zsh-dirhist.**

The main differences are:

- `dirhist` is written in Rust for better performance.
- History file is configurable via `ZSH_DIRHIST_FILE` environment variable.
- Zsh option `HIST_IGNORE_SPACE` is respected.
- Zsh option `EXTENDED_HISTORY` is respected.

---

zsh-dirhist is a zsh plugin giving you a history which is sensitive
to the directory you are in. It implements forward/backward navigation as well
as substring search in a directory sensitive manner.

Since zsh-dirhist includes [zsh-history-substring-search](https://github.com/zsh-users/zsh-history-substring-search),
do not load zsh-history-substring-search when loading this plugin.

Behavior:

- Commands executed in the current directory will pop up first when navigating
  the history or using substring search.
- A substring unknown in the current directory, will be searched for globally
  (it falls back to the normal substring search behavior).

Since the plugin creates its own history (by default in `~/.directory_history`),
it needs some time to fill up and become useful.

Installation
------------

The plugin consists of a Zsh script and a Rust command to parse the history.
In order to compile the Rust program, you need Cargo to be available.

### Manual

Clone zsh-dirhist to your preferred location:

    git clone https://github.com/cassava/zsh-dirhist
    cd zsh-dirhist
    cargo install --path .

Then activate the plugin by appending the following line to your `.zshrc` file

    source /path/to/zsh-dirhist/init.zsh

Finally, bind keyboard shortcuts in your `.zshrc`:

    # Up and Down navigate the directory history
    bindkey '^[[A' directory-history-search-forward
    bindkey '^[[B' directory-history-search-backward

    # Shift+Up and Down navigate the global history
    bindkey '^[[1;2A' history-substring-search-up
    bindkey '^[[1;2B' history-substring-search-down

### Zim

Add the following to your `.zimrc`:

    zmodule cassava/zsh-dirhist --init 'cargo install --path .'

Alternatively, you can use:

    zmodule cassava/zsh-dirhist --on-pull 'cargo install --force --path .'

Configure your bindings in `.zshrc`:

    # Up and Down navigate the directory history
    bindkey '^[[A' directory-history-search-forward
    bindkey '^[[B' directory-history-search-backward

    # Shift+Up and Down navigate the global history
    bindkey '^[[1;2A' history-substring-search-up
    bindkey '^[[1;2B' history-substring-search-down

You may have to put these inside the `zvm_after_init()` user-defined function,
depending on your setup and other plugins.

Troubleshooting
---------------

It is possible that bindings `\e[A` and `\e[B` will not work for you.
Look [here](https://github.com/zsh-users/zsh-history-substring-search) for more information.

For more information on how to configure substring search, take a look here:
https://github.com/zsh-users/zsh-history-substring-search
