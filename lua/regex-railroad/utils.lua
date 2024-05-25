local M = {}

-- Imports
local config = require("regex-railroad.config")
local consts = require("regex-railroad.consts")


--- Download and install binary from Github release
function M.install_binary()
    local tag = config.opts.tag
    local command = string.format("../../build.sh %s", tag)
    local _, _, code = os.execute(command)
    if code ~= 0 then
        vim.api.nvim_command(
            string.format(
                "echo \"%s (see https://github.com/rclawlor/regex-railroad.nvim/releases)\"",
                string.format(consts.wget_errors[code], tag)
            )
        )
    end
end


-- Check if the file exists
local function file_exists(file)
    local f = io.open(file, "rb")
    if f then
        f:close()
    end
    return f ~= nil
end

-- get all lines from a file, returns an empty
-- list/table if the file does not exist
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
