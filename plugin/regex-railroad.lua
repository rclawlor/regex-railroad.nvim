if vim.g.loaded_regex_railroad == 1 then
  return
end
vim.g.loaded_regex_railroad = 1

vim.api.nvim_create_user_command(
    "RegexRailroad",
    function()
        require("regex-railroad.command").run_diagram_command()
    end,
    {}
)

vim.api.nvim_create_user_command(
    "RegexText",
    function()
        require("regex-railroad.command").run_text_command()
    end,
    {}
)

vim.api.nvim_create_user_command(
    "UpdateRegexRailroad",
    function()
        require("regex-railroad.utils").install_binary()
    end,
    {}
)
