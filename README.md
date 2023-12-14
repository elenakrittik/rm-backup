# `rm-backup`

A minimal command line tool to backup any files you `rm`.

## Installation

Use `cargo install rm-backup`, or `binstall` if you have it.

## Usage

Intended usage is to run `rm-backup` before actual `rm` with
equal arguments. For example, here is a working example for `fish`:

```fish
function rm;
  rm-backup $argv;
  if not contains -- "--get" $argv
    command rm $argv
  end
end

# run `funcsave rm` to persist
```

`rm-backup` will backup all specified files (and folders if
`--recursive`/`-r`/`-R` is specified) to `~/.cache/rm-backup/`.

After that you can read logs located in the same folder to determine
under which name the folder you're looking for was saved. You can
also use `cat $(rm-backup --get)` to quickly read latest log.

## Contributing

Huh?
