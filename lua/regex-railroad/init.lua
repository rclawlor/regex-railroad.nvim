local M = {}

-- Imports
local config = require("regex-railroad.config")

--- Setup function for the regex-railroad.nvim plugin
---
--- @param opts table
function M.setup(opts)
    opts = opts or {}

    config.set_defaults(opts)
end


return M
