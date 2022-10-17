param(
[string]$toolName
)

# Download the script
$downloadUrl = "https://raw.githubusercontent.com/bvanseg/ccake-install-repo/master/windows/$toolName.ps1"
$downloadPath = "$PSScriptRoot\$toolName.ps1".substring(4) # Substring to remove wildcard pathing prefix.
Invoke-WebRequest $downloadUrl -OutFile $downloadPath

# Run the script
& $downloadPath