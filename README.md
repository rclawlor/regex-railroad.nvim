# regex-railroad.nvim

![rust workflow](https://github.com/rclawlor/regex-railroad.nvim/actions/workflows/rust.yml/badge.svg)
![lua workflow](https://github.com/rclawlor/regex-railroad.nvim/actions/workflows/lua.yml/badge.svg)


`regex-railroad.nvim` generates useful text and diagrams to help explain regular expressions in your code.

## Getting started
### Required dependencies
- [nvim-treesitter/nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) to extract the regular expression text

### Installation
#### lazy.nvim
```lua
-- plugins/regex-railroad.lua:
return {
    "rclawlor/regex-railroad.nvim",
    tag = "0.0.1",
    dependencies = { "nvim-treesitter/nvim-treesitter" }
}
```

## Usage
Use `:RegexText` to generate a text description of the regular expression under your cursor, or `:RegexRailroad` to instead generate a railroad diagram!

![regex-railroad](https://github.com/rclawlor/regex-railroad.nvim/assets/73249568/252a4bb9-4fd8-44e5-ab26-ba694e6049b1)

To remap the functions to something more convenient, use the following:
```lua
vim.api.nvim_set_keymap("n", "<C-x>", "<cmd>RegexText<CR>", {noremap = true, silent = true})
vim.api.nvim_set_keymap("n", "<C-s>", "<cmd>RegexRailroad<CR>", {noremap = true, silent = true})
```

## Customisation
This section explains the available options for configuring `regex-railroad.nvim`

### Setup function
```lua
require("regex-railroad").setup({
    --- Github release of plugin
    tag = "v0.0.1",
    --- Highlight group used in :RegexText
    highlight = {
        bold = true,
        fg = "fg",
        bg = "bg"
    }})
```

