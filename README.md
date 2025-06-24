# diff
diff is a binary that shows the changes that occurred to a file given the copy right before the change


## Instalation and Usage

clone this repository and then run this command to build the project (you'll need to have rust installed in your system)

```bash
    cargo build -r
```

then go to the `target\release` directory there you'll find the `diff` binary

to use it just pass to it to files one represent the new version of it and the second represent to older one before the change

the `--json` flage just format the output to a JSON representation