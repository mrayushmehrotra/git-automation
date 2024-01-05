## WHY GIT AUTOMATION

not all people uses VsCode, because vs code is heavy memory tasking thread for our pc
and sometimes can slow down our pc, alot , i mean alot ;).
that's why this simple git intergration makes all your waiting and heavy lifting
away from you, because personally i found doing all the <i>git init, git add, yada yada</i>
commands takes your 20 30 seconds on inititalising the project,
And While Working it also takes some 20 second to push changes.

```
# clone the repo and get into it
git clone https://github.com/mrayushmehrotra/git-automation

cd git-automation

# build the project and run

cargo build

cargo run <remote_url>

```

## if you want to globally install this project to use it anywhere use this command after building this project

```
cargo install --path .
```

