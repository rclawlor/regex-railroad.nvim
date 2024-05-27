local M = {}

--- User home directory
M.home_directory = string.format(
    "%s", os.getenv("HOME")
)

--- Root directory of plugin
M.root_directory = string.format(
    "%s/nvim/lazy/regex-railroad.nvim",
    os.getenv("XDG_CONFIG_HOME") or string.format("%s/.local/share", M.home_directory)
)

--- Rust binary location
M.binary_location = string.format(
    "%s/regex-railroad",
    M.root_directory
)

--- Working directory for development
M.dev_directory = vim.fn.getcwd()

--- Rust binary location for development
M.dev_binary_location = string.format(
    "%s/target/release/regex-railroad",
    M.dev_directory
)

--- A mapping of wget error codes to useful user message
M.wget_errors = {
    [3] = "I/O error when writing %s regex-raiload binary",
    [4] = "Network failure when downloading %s regex-railroad binary",
    [5] = "SSL verification failure when downloading %s regex-railroad binary",
    [6] = "Username/password authentication failure when downloading %s regex-raiload binary",
    [7] = "wget protocal error when downloading %s regex-raiload binary",
    [8] = "Could not find release %s"
}


return M
