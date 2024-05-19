if vim.g.loaded_regex_railroad == 1 then
  return
end
vim.g.loaded_regex_railroad = 1

local job = require('regex-railroad.job')

vim.api.nvim_create_user_command(
    "RegexRailroad",
    function(opts)
        require("regex-railroad.command").run_diagram_command(opts)
    end,
    {}
)

vim.api.nvim_create_user_command(
    "RegexText",
    function(opts)
        require("regex-railroad.command").run_text_command(opts)
    end,
    {}
)
