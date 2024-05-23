local M = {}

-- Imports
local job = require("regex-railroad.job")
local config = require("regex-railroad.config")

-- Variables
local jobid


--- Send RPC command to generate and display a railroad diagram of the regular expression
---
--- @param filename string name of current file
--- @param text string text containing regular expression
local function regex_railroad(filename, text)
    vim.api.nvim_call_function(
        "rpcnotify",
        {
            jobid,
            "regexrailroad",
            { filename, text }
        }
    )
end


--- Send RPC command to generate and display a text description of the regular expression
---
--- @param filename string name of current file
--- @param text string text containing regular expression
local function regex_text(filename, text)
    vim.api.nvim_call_function(
        "rpcnotify",
        {
            jobid,
            "regextext",
            { filename, text }
        }
    )
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
local function win_open_autocmd(win_id)
    vim.api.nvim_create_autocmd("WinNew", {
        once = true,
        callback = function()
            local fwin = vim.api.nvim_get_current_win()
            local fbuf = vim.api.nvim_get_current_buf()

            vim.api.nvim_set_current_win(win_id)
            configure_floating_window(fwin, fbuf)
        end,
    })
end


--- Runs when :RegexRailroad command executed
function M.run_diagram_command()
    -- Use treesitter to extract regex text
    local line
    local node = vim.treesitter.get_node()
    if node then
        line = vim.treesitter.get_node_text(node, 0)
    else
        line = ""
    end
    local filename = vim.api.nvim_buf_get_name(0)
    jobid = job.attach(filename)
    regex_railroad(filename, line)
end


--- Runs when :RegexText command executed
function M.run_text_command()
    -- Use treesitter to extract regex text
    local line
    local node = vim.treesitter.get_node()
    if node then
        line = vim.treesitter.get_node_text(node, 0)
    else
        line = ""
    end

    -- Use filename to extract current language
    local filename = vim.api.nvim_buf_get_name(0)

    -- Set highlight group from config
    vim.api.nvim_set_hl(
        0,
        "RegexHighlight",
        config.opts.highlight
    )

    local current_win = vim.api.nvim_get_current_win()

    jobid = job.attach(filename)
    regex_text(filename, line)

    win_open_autocmd(current_win)
end

return M
