$strippedFileName = "solodndapp"
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$ZipName = "$($strippedFileName)_$($timestamp)$($extension).zip"

# Resolve the source path and check it exists
$SourcePath = (Resolve-Path -Path "C:\dev\SoloDnDApp").Path

if (-not (Test-Path -LiteralPath $SourcePath -PathType Container)) {
    throw "SourcePath '$SourcePath' is not a valid directory."
}

# Folder name of the source (used inside the zip)
$folderName = Split-Path -Path $SourcePath -Leaf

# Where to put the zip: the directory the script is RUN from (current location)
$outputDir = (Get-Location)

if (-not $ZipName -or [string]::IsNullOrWhiteSpace($ZipName)) {
    $timestamp = Get-Date -Format 'yyyyMMdd_HHmmss'
    $ZipName = "$folderName-$timestamp.zip"
}

$DestinationPath = Join-Path -Path $outputDir -ChildPath $ZipName

if (Test-Path -LiteralPath $DestinationPath) {
    throw "Destination zip already exists: $DestinationPath"
}

Write-Host "Source folder : $SourcePath"
Write-Host "Run folder    : $outputDir"
Write-Host "Output zip    : $DestinationPath"
Write-Host "Excluding     : *.pdf files and the .git folder"
Write-Host ""

# Normalize root path (for relative paths in the zip)
$root     = $SourcePath.TrimEnd('\','/')
$rootName = Split-Path -Path $root -Leaf

# 1) Collect the files BEFORE creating the zip so the zip never includes itself
$files = Get-ChildItem -LiteralPath $root -Recurse -File | Where-Object {
    # Exclude PDFs
    $_.Extension -ne '.pdf' -and
    # Exclude anything in a ".git" folder (Windows and Unix-style paths)
    $_.FullName -notmatch '(^|[\\/])\.git([\\/])'
}

# 2) Now create and populate the zip
Add-Type -AssemblyName System.IO.Compression.FileSystem

$zip = [System.IO.Compression.ZipFile]::Open($DestinationPath, 'Create')

try {
    foreach ($file in $files) {
        # Build the path inside the zip, preserving directory structure
        $relative = $file.FullName.Substring($root.Length).TrimStart('\','/')
        # Include the top-level folder name in the archive
        $entryName = Join-Path $rootName $relative

        [System.IO.Compression.ZipFileExtensions]::CreateEntryFromFile(
            $zip,
            $file.FullName,
            $entryName,
            [System.IO.Compression.CompressionLevel]::Optimal
        ) | Out-Null
    }
}
finally {
    $zip.Dispose()
}

Write-Host ""
Write-Host "✅ Archive created successfully:"
Write-Host "   $DestinationPath"