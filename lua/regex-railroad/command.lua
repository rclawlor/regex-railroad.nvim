local job = require("regex-railroad.job")
local command = {}
local buf
local win
local jobid

local function regex_railroad(filename, row, col, length, text)
    vim.api.nvim_call_function(
        "rpcnotify",
        {
            jobid,
            "regexrailroad",
            { filename, row, col, length, text }
        }
    )
end

local function regex_text(filename, row, col, length, text)
    vim.api.nvim_call_function(
        "rpcnotify",
        {
            jobid,
            "regextext",
            { filename, row, col, length, text }
        }
    )
end


local function send_echo(text)
   vim.api.nvim_call_function(
        "rpcnotify",
        {
            jobid,
            "echo",
            { text }
        }
    )
end

local function open_window()
    buf = vim.api.nvim_create_buf(false, true) -- create new empty buffer

    -- get dimensions
    -- TODO: below functions are deprecated and need replaced
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

function command.run_diagram_command(args)
    -- Use treesitter to extract regex text
    local line
    local row
    local col
    local length

    local node = vim.treesitter.get_node()
    if node then
        row, col, length = node:start()
        line = vim.treesitter.get_node_text(node, 0)
    else
        row = 0
        col = 0
        length = 0
        line = ""
    end
    local filename = vim.api.nvim_buf_get_name(0)
    buf = open_window()
    jobid = job.attach(buf)
    regex_railroad(filename, row, col, length, line)
end

function command.run_text_command(args)
    -- Use treesitter to extract regex text
    local line
    local row
    local col
    local length

    local node = vim.treesitter.get_node()
    if node then
        row, col, length = node:start()
        line = vim.treesitter.get_node_text(node, 0)
    else
        row = 0
        col = 0
        length = 0
        line = ""
    end
    local filename = vim.api.nvim_buf_get_name(0)
    local current_buf = vim.api.nvim_get_current_buf()
    jobid = job.attach(buf)
    regex_text(filename, row, col, length, line)
end

return command
