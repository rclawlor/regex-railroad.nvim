param (
    [string]$a = ""
    [string]$b = ""
)

$URL = https://github.com/rclawlor/regex-railroad.nvim/releases/download/$b/regex-railroad-windows.exe

Invoke-WebRequest $URL -OutFile $a/regex-railroad-windows.exe

