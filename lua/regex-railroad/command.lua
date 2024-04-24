local job = require("regex-railroad.job")
local command = {}
local buf
local win
local jobid

local function send_msg(position, msg)
    vim.api.nvim_call_function(
        "rpcnotify",
        {
            jobid,
            "echo",
            { position, msg }
        }
    )
end

local function open_window()
    buf = vim.api.nvim_create_buf(false, true) -- create new empty buffer

    -- get dimensions
    local width = vim.api.nvim_get_option("columns")
    local height = vim.api.nvim_get_option("lines")

    -- calculate our floating window size
    local win_height = math.ceil(height * 0.4 - 4)
    local win_width = math.ceil(width * 0.9)

    -- and its starting position
    local row = math.ceil(3 * (height - win_height) / 4 - 1)
    local col = math.ceil((width - win_width) / 2)

    local opts = {
        title = "Regex Railroad",
        border = "rounded",
        style = "minimal",
        relative = "editor",
        width = win_width,
        height = win_height,
        row = row,
        col = col
    }

    win = vim.api.nvim_open_win(buf, true, opts)

    return buf
end

local function run_command(args)
    local line = vim.api.nvim_get_current_line()
    local _, col = table.unpack(vim.api.nvim_win_get_cursor(0))
    buf = open_window()
    jobid = job.attach(buf)
    send_msg(col, line)
end


function command.load_command(cmd, ...)
    local args = { ... }
    run_command(args)
end

return command
