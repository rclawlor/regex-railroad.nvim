local regex_railroad = { }


--- Setup function for the regex-railroad.nvim plugin
---
--- @param opts table
function regex_railroad.setup(opts)
    opts = opts or {}

    require("regex-railroad.config").set_defaults(opts)
end

return regex_railroad
