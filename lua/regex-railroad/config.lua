local defaults = require("regex-railroad.defaults")
local config = {}


--- Set config to user provided values, using default config if nil
---
--- @param user table: a table where keys are the names of options,
---     and values are the ones the user wants
function config.set_defaults(user)
    user = vim.F.if_nil(user, {})

    for key, value in pairs(defaults) do
        if user[key] == nil then
            config[key] = value
        else
            config[key] = user[key]
        end
    end
end


return config
