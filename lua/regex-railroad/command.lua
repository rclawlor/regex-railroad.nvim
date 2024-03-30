local command = {}
local buf
local win

local function open_window()
    buf = vim.api.nvim_create_buf(false, true) -- create new empty buffer
--    vim.api.nvim_buf_set_option(buf, 'bufhidden', 'wipe')

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
    -- vim.api.nvim_win_set_option(win, 'cursorline', true) -- it highlight line with the cursor on it
end

local function run_command(args)
    open_window()
end


function command.load_command(cmd, ...)
    local args = { ... }
    run_command(args)
end

return command
