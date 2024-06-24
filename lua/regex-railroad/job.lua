local M = {}

-- Imports
local config = require("regex-railroad.config")
local consts = require("regex-railroad.consts")

-- Variables
local binary = nil
M.jobid = nil


--- Attach RPC job to current file
---
--- @param filename string current filename
function M.attach(filename)
    if binary == nil then
        if config.opts.dev then
            binary = consts.dev_binary_location()
        else
            binary = consts.binary_location()
        end
    end

    local binlist
    if filename == nil or filename == "" then
        binlist = { binary }
    else
        binlist = { binary, filename }
    end

    M.jobid = vim.api.nvim_call_function(
        "jobstart",
        {
            binlist,
            { rpc=true }
        }
    )

    if M.jobid == 0 then
        return false
    elseif M.jobid == -1 then
        return false
    else
        return M.jobid
    end
end


--- Detach RPC job
function M.detach()
    if not M.jobid then
        return false
    else
        vim.api.nvim_call_function(
            "rpcnotify",
            {
                M.jobid,
                "quit"
            }
        )
        return true
    end
end


return M
