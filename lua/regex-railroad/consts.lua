local M = {}

--- User home directory
M.home_directory = string.format(
    "%s", os.getenv("HOME")
)

--- Root directory of plugin
function M.root_directory()
    local config_dir
    if vim.fn.has("win32") == 1 then
        config_dir = os.getenv("LOCALAPPDATA")
        return string.format(
            "%s/nvim-data/lazy/regex-railroad.nvim",
            config_dir
        )
    elseif vim.fn.has("linux") == 1 then
        config_dir = os.getenv("XDG_DATA_HOME") or string.format("%s/.local/share", M.home_directory)
        return string.format(
            "%s/nvim/lazy/regex-railroad.nvim",
            config_dir
        )
    elseif vim.fn.has("mac") == 1 then
        config_dir = os.getenv("XDG_DATA_HOME") or string.format("%s/.local/share", M.home_directory)
        return string.format(
            "%s/nvim/lazy/regex-railroad.nvim",
            config_dir
        )
    else
        vim.api.nvim_command(
            "echohl ErrorMsg | echo \"OS not recognised - only Linux, Mac and Windows supported\" | echohl None"
        )
        return nil
    end
end

--- Rust binary name
function M.binary_name()
    if vim.fn.has("win32") == 1 then
        return "regex-railroad-windows.exe"
    elseif vim.fn.has("linux") == 1 then
        return "regex-railroad-linux"
    elseif vim.fn.has("mac") == 1 then
        return "regex-railroad-mac"
    else
        vim.api.nvim_command(
            "echohl ErrorMsg | echo \"OS not recognised - only Linux, Mac and Windows supported\" | echohl None"
        )
        return nil
    end
end

--- Rust binary location
function M.binary_location()
    local binary_name = M.binary_name()

    if binary_name ~= nil then
        return string.format(
            "%s/%s",
            M.root_directory(),
            binary_name
        )
    else
        return nil
    end
end

--- Working directory for development
M.dev_directory = vim.fn.getcwd()

--- Rust binary location for development
function M.dev_binary_location()
    local extension = ""
    if vim.fn.has("win32") == 1 then
        extension = ".exe"
    end

    return string.format(
        "%s/target/debug/regex-railroad%s",
        M.dev_directory,
        extension
    )
end

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
