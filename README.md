# filepick

A native GUI-based file picker. Outputs selected files to stdout and optionally copies them to the working directory.

## Terminal Use

Selected files are outputted to stdout one per line (or separated by \0 if --null option is included). This allows piping to other commands.

Some examples in a Bash terminal:
```bash
filepick | tar cfz foo.tar.gz --files-from=-
filepick | xargs cp -t backup/
filepick | xargs sha256sum
filepick --null | xargs -0 grep "TODO"
```

Or in PowerShell:
```PowerShell
filepick | Copy-Item -Destination backup
filepick | Select-String "TODO"
```

## File Explorer

This program can be added to the context menu in file explorer to allow functionality similar to PowerToys New+ but with traversal of the "templates" folder. Compile with the no_terminal feature (and optionally always_copy) and then add the appropriate items to `HKEY_CLASSES_ROOT\Directory\Background\shell` to make it appear as an option in the context menu in explorer. Create or pick a template folder and make sure to include the path as an argument in the command registry item.

## Features

| **Feature** | **Description**                                                                                    |
|--------------|----------------------------------------------------------------------------------------------------|
| no_terminal | No terminal window will be created when starting the application. Will no longer output to stdout. |
| default_copy | Acts as if `--copy-to-cwd` option is always included.                                             |
| always_copy | Applies default_copy feature and acts as if `--overwrite-dest` option is always included         |