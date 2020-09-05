---
title: "Vim and Tmux as Alternative to Juno for Julia Development"
date: 2020-02-15T12:51:48+01:00
features:
    - Highlight
---

I spend a lot of time coding in Julia at the moment. And while Juno is fine mostly, I was exploring a more lightweight (and fun) alternative based on Vim, Tmux, and the Julia REPL.

## Integrating the Julia REPL with Vim

Following [this post](http://www.serhatcevikel.com/?p=56) by Serhat Cevikel about Vim as an IDE for Julia, I set up a similar configuration with *Tmux* instead of *Screen*. As in the original post, we need the following prerequisites:

* [Vim](https://www.vim.org/)
* [Tmux](https://github.com/tmux/tmux) 
* [vim-slime](https://github.com/jpalardy/vim-slime) plugin
* [Julia](https://julialang.org/downloads/)

After installing all required packages, we need to configure Vim by adding the following lines to our *.vimrc*:

```vim
" set slime target (tmux instead of screen)
let g:slime_target = "tmux"
" set target pane that code is sent to (optional)
let g:slime_default_config = {"socket_name": "default", "target_pane": "0.1"}
```

The optional second line assumes that Tmux window 0 has at least two panes and the Julia REPL runs in pane 1. To configure this on startup, I wrote a simple bash script that creates a Tmux window with two panes: a Vim session with the file argument provided to the script, and the Julia REPL that will be the target of *vim-slime*.

```bash
#!/bin/bash
tmux new \; \
	send-keys "vim $1" Enter \; \
	split-window -v \; \
	rename-window julia \; \
	send-keys "julia" Enter \; \
	select-pane -t 0 \; 
```

After opening a file with this script, we can select Julia code in Vim's visual mode and send it to the REPL for execution by pressing `C-c C-c` (you can keep pressing control and tap `c` twice). The first time this is done in a session, we need to confirm the default slime configuration we set in our .vimrc by pressing enter twice. 

For long stack traces, we'll want to scroll the REPL pane. In Tmux you can scroll with the navigation keys after entering the scroll mode by typing `C-b [` from within the pane. You can quit this mode by typing `q`. Since this is inconvenient, especially on a German keyboard, I enabled mouse scrolling with the following line in *.tmux.config*:

```bash
set -g mouse on
```
With this option you need to press shift while selecting text in order to copy from the pane.


## Language Support

There is a very nice plugin called [julia-vim](https://github.com/JuliaEditorSupport/julia-vim) which provides basic support for editing Julia files (indentation, syntax highlighting) and additional functionality such as converting LaTeX to Unicode symbols and block-wise movements.

Further language support (autcomplete, goto definition, documentation on hover) should be available via [LanguageServer.jl](https://github.com/julia-vscode/LanguageServer.jl), an implementation of Microsoft's [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) for Julia. I briefly tried to set it up with [LanguageClient-neovim](https://github.com/autozimu/LanguageClient-neovim/) and [ncm2](https://github.com/ncm2/ncm2), but alas, without success. I'll have to try again in a quiet hour.


## File Explorer

Another central feature of Atom and most IDEs is a file explorer. For Vim there is a nice plugin called [NERDTree](https://github.com/preservim/nerdtree). For convenience I defined two key mappings in my *.vimrc* to access basic features:

```vim
" open/close NERDTree
map <F2> :NERDTreeToggle<CR>
" reveal file in tree
map <F3> :NERDTreeFind<CR>
```

The first command toggles NERDTree in a little window at the left side. The second command is NERDTree's equivalent of Atom's "reveal in tree" functionality.


## Git Integration

I mostly use Git via the command line, together with [gitk](https://git-scm.com/docs/gitk), so for me being able to split the bottom pane (`C-b %`) vertically and having a terminal on the right side is already an improvement. I know that there are several popular plugins for working with Git from within Vim, namely 

* [vim-fugitive](https://github.com/tpope/vim-fugitive), a seemingly quite comprehensive Git wrapper,
* [vim-gitgutter](https://github.com/airblade/vim-gitgutter), a plugin that shows git diffs and has a lot of other functionality, and
* [nerdtree-git-plugin](https://github.com/Xuyuanp/nerdtree-git-plugin), which extends NERDTree to show the Git status.

However, so far I haven't tried any of them.


## Killing the Monster

This line in *.tmux.conf* creates a shortcut for killing the active session:
```bash
bind X confirm-before kill-session
```
To load Tmux config changes immediately (without restarting the server), you can run `tmux source-file ~/.tmux.conf` on the command line. Afterwards you can leave your custom IDE by typing `C-b X` and confirming your action.