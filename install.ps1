<#
.SYNOPSIS
Installs filepick on Windows.
.DESCRIPTION
Copies a built filepick.exe into a user install location, optionally adds it to PATH,
and optionally registers a File Explorer background context menu entry.
.PARAMETER AddToPath
Add the install directory to the PATH environment variable.
.PARAMETER AddExplorerContextMenu
Register filepick in the File Explorer background context menu.
.PARAMETER ExePath
Explicit path to an existing built filepick.exe.
.PARAMETER InstallDir
Explicit target installation directory.
.PARAMETER CommandName
Executable name to install (default: filepick).
#>
[CmdletBinding()]
param(
    [switch]$AddToPath,
    [switch]$AddExplorerContextMenu,
    [string]$ExePath,
    [string]$InstallDir,
    [string]$CommandName = "filepick"
)

Set-StrictMode -Version Latest

function Resolve-SourceExe {
    param([string]$path)

    if ($path) {
        if (Test-Path $path -PathType Leaf) {
            return (Resolve-Path $path).Path
        }
        throw "Specified executable path does not exist: $path"
    }

    $candidates = @(
        Join-Path $PSScriptRoot "target\release\$CommandName.exe",
        Join-Path $PSScriptRoot "target\debug\$CommandName.exe"
    )

    foreach ($candidate in $candidates) {
        if (Test-Path $candidate -PathType Leaf) {
            return (Resolve-Path $candidate).Path
        }
    }

    throw "Could not find built executable. Build the project first, or pass -ExePath with a valid path."
}

function Add-DirectoryToPath {
    param([string]$pathToAdd)

    $scope = [EnvironmentVariableTarget]::User
    $currentPath = [Environment]::GetEnvironmentVariable('Path', $scope) -or ''

    $entries = $currentPath -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne '' }
    $normalized = $pathToAdd.Trim('"').TrimEnd('\').ToLowerInvariant()
    $existing = $entries | ForEach-Object {
        $_.Trim('"').TrimEnd('\').ToLowerInvariant()
    }

    if ($existing -contains $normalized) {
        Write-Host "Path already contains: $pathToAdd"
        return
    }

    $newPath = ($entries + $pathToAdd) -join ';'
    [Environment]::SetEnvironmentVariable('Path', $newPath, $scope)
    $env:Path += ";$pathToAdd"
    Write-Host "Updated PATH ($scope) to include: $pathToAdd"
}

function Create-ContextMenuEntry {
    param([string]$exePath)

    $rootKey = "HKCU:\Software\Classes\Directory\Background\shell\$CommandName"
    $commandKeyPath = "$rootKey\command"

    New-Item -Path "$rootKey" -Force | Out-Null
    Set-ItemProperty -Path "$rootKey" -Name 'MUIVerb' -Value $CommandName -Force
    Set-ItemProperty -Path "$rootKey" -Name 'Icon' -Value $exePath -Force

    New-Item -Path "$commandKeyPath" -Force | Out-Null
    Set-Item -Path "$commandKeyPath" -Value "`"$exePath`" `"%V`"" -Force
    Write-Host "Created Explorer background context menu entry at $commandKeyPath"
}

try {
    $sourceExe = Resolve-SourceExe -path $ExePath
} catch {
    Write-Error $_
    exit 1
}

$targetRoot = if ($InstallDir) {
    $InstallDir
} else {
    Join-Path $env:LOCALAPPDATA "Programs\$CommandName"
}

New-Item -ItemType Directory -Path $targetRoot -Force | Out-Null
$targetExe = Join-Path $targetRoot "$CommandName.exe"
Copy-Item -Path $sourceExe -Destination $targetExe -Force
Write-Host "Installed $CommandName to $targetExe"

if ($AddToPath) {
    Add-DirectoryToPath -pathToAdd $targetRoot
}

if ($AddExplorerContextMenu) {
    Create-ContextMenuEntry -exePath $targetExe
    Write-Host "Restart File Explorer or log out and back in to see the new context menu entry."
}

Write-Host "Installation complete."
