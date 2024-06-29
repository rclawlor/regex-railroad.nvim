local M = {}

-- Imports
local config = require("regex-railroad.config")
local consts = require("regex-railroad.consts")


--- Download and install binary from Github release
function M.install_binary()
    local tag = config.opts.tag
    local directory = config.opts.dev and consts.dev_directory() or consts.root_directory()
    vim.api.nvim_command(
            string.format(
                "echo \"Installing regex-railroad %s\"",
                tag
            )
        )
    local command
    if vim.fn.has("win32") == 1 then
        command = string.format(
            "%/build.ps1 -a %s -b %s > nul 2>&1",
            directory,
            directory,
            tag
        )
    elseif vim.fn.has("linux") == 1 then
        command = string.format(
            "%s/build.sh %s %s %s >/dev/null 2>&1",
            directory,
            directory,
            tag,
            "linux"
        )
    elseif vim.fn.has("mac") == 1 then
        command = string.format(
            "%s/build.sh %s %s %s >/dev/null 2>&1",
            directory,
            directory,
            tag,
            "macos"
        )
    end
    local code = os.execute(command) / 256
    if not (code == 0 or code == nil) then
        vim.notify(
            string.format(
                "%s (see https://github.com/rclawlor/regex-railroad.nvim/releases)",
                string.format(consts.wget_errors[code], tag)
            ),
            vim.log.levels.ERROR
        )
    else
        vim.notify(
            string.format(
                "Successfully installed regex-railroad %s",
                tag
            ),
            vim.log.levels.INFO
        )
    end
end


--- Check if the file exists
local function file_exists(file)
    local f = io.open(file, "rb")
    if f then
        f:close()
    end
    return f ~= nil
end

--- Get all lines from a file
---
--- @param file any file name
--- @return table lines of file
function M.lines_from_file(file)
    if not file_exists(file) then
        return {}
    end

    local lines = {}
    for line in io.lines(file) do
        lines[#lines + 1] = line
    end

    return lines
end


return M
