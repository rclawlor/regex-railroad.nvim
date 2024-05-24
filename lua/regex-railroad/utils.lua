local M = {}

-- Imports
local config = require("regex-railroad.config")


--- Download and install binary from Github release
function M.install_binary()
    -- Find user install location
    local home_dir = os.getenv("XDG_CONFIG_HOME") or "~/.local/share"
    local install_dir = string.format("{}/lazy/regex-railroad", home_dir)
    local tag = config.opts.tag

    -- Download binary from Github release
    local http = require("socket.http")
    local body, code = http.request(
        string.format(
            "https://github.com/rclawlor/regex-railroad.nvim/releases/download/{}/regex-railroad",
            tag
        )
    )
    if not body then
        error(code)
    end

    -- Save binary to lazy.nvim install location
    local f = assert(io.open(string.format("{}/regex-railroad", install_dir), "wb"))
    f:write(body)
    f:close()
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
