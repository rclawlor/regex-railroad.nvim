local command = {}
local buf
local win
local border_win

local function open_window()
    buf = vim.api.nvim_create_buf(false, true) -- create new empty buffer
    local border_buf = vim.api.nvim_create_buf(false, true)

    vim.api.nvim_buf_set_option(buf, 'bufhidden', 'wipe')

    -- get dimensions
    local width = vim.api.nvim_get_option("columns")
    local height = vim.api.nvim_get_option("lines")

    -- calculate our floating window size
    local win_height = math.ceil(height * 0.8 - 4)
    local win_width = math.ceil(width * 0.8)

    -- and its starting position
    local row = math.ceil((height - win_height) / 2 - 1)
    local col = math.ceil((width - win_width) / 2)

    local border_opts = {
        style = "minimal",
        relative = "editor",
        width = win_width + 2,
        height = win_height + 2,
        row = row - 1,
        col = col - 1
    }

    local opts = {
        style = "minimal",
        relative = "editor",
        width = win_width,
        height = win_height,
        row = row,
        col = col
    }

    local border_lines = { '╔' .. string.rep('═', win_width) .. '╗' }
    local middle_line = '║' .. string.rep(' ', win_width) .. '║'
    for i=1, win_height do
        table.insert(border_lines, middle_line)
    end
    table.insert(border_lines, '╚' .. string.rep('═', win_width) .. '╝')
    vim.api.nvim_buf_set_lines(border_buf, 0, -1, false, border_lines)

    border_win = vim.api.nvim_open_win(border_buf, true, border_opts)
    win = vim.api.nvim_open_win(buf, true, opts)
    vim.api.nvim_command('au BufWipeout <buffer> exe "silent bwipeout! "'..border_buf)

    vim.api.nvim_win_set_option(win, 'cursorline', true) -- it highlight line with the cursor on it
end

local function run_command(args)
    open_window()
end


function command.load_command(cmd, ...)
    local args = { ... }
    run_command(args)
end

return command
