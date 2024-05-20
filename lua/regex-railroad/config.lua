local M = {}

-- Imports
local defaults = require("regex-railroad.defaults")

-- Variables
M.opts = {}


--- Set config to user provided values, using default config if nil
---
--- @param user table: a table where keys are the names of options,
---     and values are the ones the user wants
function M.set_defaults(user)
    user = vim.F.if_nil(user, {})

    for key, value in pairs(defaults) do
        if user[key] == nil then
            M.opts[key] = value
        else
            M.opts[key] = user[key]
        end
    end
end


return M
