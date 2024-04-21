local jobids = require("regex-railroad.job").jobids
local job = require("regex-railroad.job")
local command = {}
local buf
local win

local function send_msg()
    vim.api.nvim_call_function(
        "rpcnotify",
        {
            jobids[buf],
            "echo",
            5
        }
    )
    -- vim.api.nvim_command('wincmd p')
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
end

local function run_command(args)
    open_window()
    job.attach()
    send_msg()
end


function command.load_command(cmd, ...)
    local args = { ... }
    run_command(args)
end

return command
