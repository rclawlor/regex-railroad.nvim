local job = require("regex-railroad.job")
local command = {}
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

--- Closes the preview window
---
---@param win_id integer ID of floating window
---@param buf_ids table|nil optional list of ignored buffers
local function close_preview_window(win_id, buf_ids)
    vim.schedule(
        function()
            -- exit if we are in one of ignored buffers
            if buf_ids and vim.list_contains(buf_ids, vim.api.nvim_get_current_buf()) then
                return
            end

            local augroup = "floating_window_" .. win_id
            pcall(vim.api.nvim_del_augroup_by_name, augroup)
            pcall(vim.api.nvim_win_close, win_id, true)
        end
    )
end

--- Creates autocommand to close floating window based on events
---
--- @param events table list of events
--- @param win_id integer ID of floating window
--- @param buf_ids table IDs of buffers where floating window can be seen
local function close_win_autocmd(events, win_id, buf_ids)
    local augroup = vim.api.nvim_create_augroup("floating_window_" .. win_id, {
        clear = true,
    })
    -- close the preview window when entered a buffer that is not
    -- the floating window buffer or the buffer that spawned it
    vim.api.nvim_create_autocmd("BufEnter", {
        group = augroup,
        callback = function()
            close_preview_window(win_id, buf_ids)
        end,
    })

    if #events > 0 then
        vim.api.nvim_create_autocmd(events, {
            group = augroup,
            callback = function()
                close_preview_window(win_id)
            end,
        })
    end
end

--- Configures floating window and sets up autocommand
---
--- @param win_id integer ID of floating window
--- @param buf_id integer ID of floating buffer
local function configure_floating_window(win_id, buf_id)
    -- Disable folding on current window
    vim.wo[win_id].foldenable = false

    vim.bo[buf_id].bufhidden = "wipe"

    vim.api.nvim_buf_set_keymap(
        buf_id,
        'n',
        'q',
        '<cmd>bdelete<cr>',
        { silent = true, noremap = true, nowait = true }
    )

    local close_events = { 'CursorMoved' }
    close_win_autocmd(close_events, win_id, { buf_id })
end

--- Sets up autocommand to wait for floating window open
---
--- @param win_id integer ID of current window
--- @param buf_id integer ID of current buffer
local function win_open_autocmd(win_id, buf_id)
    vim.api.nvim_create_autocmd("WinNew", {
        once = true,
        callback = function()
            local fwin = vim.api.nvim_get_current_win()
            local fbuf = vim.api.nvim_get_current_buf()

            vim.api.nvim_set_current_win(win_id)
            vim.api.nvim_command(string.format("echomsg '%d -> %d'", buf_id, fbuf))
            configure_floating_window(fwin, fbuf)
        end,
    })
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
    local current_win = vim.api.nvim_get_current_win()

    jobid = job.attach(buf)
    regex_text(filename, row, col, length, line)

    win_open_autocmd(current_win, current_buf)
end

return command
