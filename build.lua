-- Imports
local utils = require("regex-railroad.utils")

vim.schedule(
    function()
        utils.install_binary()
    end
)
