local locate_binary = require('regex-railroad.utils').locate_binary

-- Holds the binary to start
local binary = nil

-- Holds buffer -> jobid associations
local jobid = nil

local function attach(buf, filename)
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
        return jobid
    end
end


local function detach(buf)
    buf = buf or vim.api.nvim_get_current_buf()

    if not jobid then
        return false
    else
        vim.api.nvim_command.call(
            "rpcnotify",
            {
                jobid,
                "quit"
            }
        )
        return true
    end
end


return {
    attach = attach,
    detach = detach,
    jobid = jobid
}

