local locate_binary = require('regex-railroad.utils').locate_binary

-- Holds the binary to start
local binary = nil

-- Holds buffer -> jobid associations
local jobids = {}

local function attach(filename)
    vim.api.nvim_command('echomsg "Attaching to file"')
    local buf = vim.api.nvim_get_current_buf()
    local jobid

    if binary == nil then
        binary = locate_binary()
    end

    local binlist
    if filename == nil or filename == "" then
        binlist = { binary }
    else
        binlist = { binary, filename }
    end

    jobid = vim.api.nvim_call_function(
        "jobstart",
        {
            binlist,
            { rpc=true }
        }
    )

    if jobid == 0 then
        return false
    elseif jobid == -1 then
        return false
    else
        jobids[buf] = jobid
        vim.api.nvim_command('echomsg "Found jobid"')
        return true
    end
end


local function detach(buf)
    buf = buf or vim.api.nvim_get_current_buf()
    local jobid = jobids[buf]

    if not jobid then
        return false
    else
        vim.api.nvim_command.call(
            "rpcnotify",
            {
                jobids[buf],
                "quit"
            }
        )
        return true
    end
end


return {
    attach = attach,
    detach = detach,
    jobids = jobids
}

