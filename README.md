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

This program can be added to the context menu in file explorer to allow functionality similar to PowerToys New+ but with traversal of the "templates" folder. Compile with the no_terminal feature (and optionally always_copy) and then add the appropriate items to `HKCU:\Software\Classes\Directory\Background\shell` to make it appear as an option in the context menu in explorer. Create or pick a template folder and make sure to include the path as an argument in the command registry item.

Recommended File Explorer installation using the installation script:
- build release with `no_terminal` and `always_copy`
```powershell
cargo build --release --features no_terminal,always_copy
```
- install as a context menu item in File Explorer
```powershell
.\install.ps1 -AddExplorerContextMenu -CommandName "Add Template" -TemplateDir "%USERPROFILE%\Templates"
```
- now add a Templates folder in `C:\Users\{Your Username}` and put some files in here (as nested as you like) to easily add from the context menu in File Explorer

## Windows Installation Script

To simplify installation on Windows, use `install.ps1`. First compile the program in release mode. Use the `no_terminal` feature to hide the visible terminal window when installing as an Explorer menu item. Then run the install script with the desired options.

Usage examples:

```powershell
# Install for current user in LocalAppData and add it to PATH
.\install.ps1 -AddToPath

# Install for current user and add a background context menu entry that always opens a specific template directory
.\install.ps1 -AddExplorerContextMenu -CommandName "Add Template" -TemplateDir "C:\path\to\templates"

# Install from a custom built executable path into a custom directory and add to PATH
.\install.ps1 -ExePath .\target\release\filepick.exe -InstallDir %USERPROFILE%\Tools\FilePick -AddToPath
```

## Features

| **Feature** | **Description**                                                                                    |
|--------------|----------------------------------------------------------------------------------------------------|
| no_terminal | No terminal window will be created when starting the application. Will no longer output to stdout. |
| default_copy | Acts as if `--copy-to-cwd` option is always included.                                             |
| always_copy | Applies default_copy feature and acts as if `--overwrite-dest` option is always included         |