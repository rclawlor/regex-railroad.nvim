local M = {}

-- Variables
M.binary_location = string.format(
    "{}/lazy/regex-railroad/regex-railroad",
    os.getenv("XDG_CONFIG_HOME") or "~/.local/share"
)

M.wget_errors = {
    [3] = "I/O error when writing %s regex-raiload binary",
    [4] = "Network failure when downloading %s regex-railroad binary",
    [5] = "SSL verification failure when downloading %s regex-railroad binary",
    [6] = "Username/password authentication failure when downloading %s regex-raiload binary",
    [7] = "wget protocal error when downloading %s regex-raiload binary",
    [8] = "Could not find release %s"
}


return M
