if vim.g.loaded_regex_railroad == 1 then
  return
end
vim.g.loaded_regex_railroad = 1

vim.api.nvim_create_user_command(
    "RegexRailroad",
    function(opts)
        require("regex-railroad.command").load_command(opts)
    end,
    {}
)
