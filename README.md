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
    tag = "0.0.3",
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
    tag = "v0.0.3",
    --- Highlight group used in :RegexText
    highlight = {
        bold = true,
        fg = "fg",
        bg = "bg"
    }})
```

## Supported Features
### Character classes
<center>

| Feature               | Example   | Supported |
|:---------------------:|:---------:|:---------:|
| Character set         | [ABC]     | &check;   |
| Negated set           | [^ABC]    | &check;   |
| Range                 | [A-Z]     | &check;   |
| Dot                   | .         | &check;   |
| Word                  | \w        | &check;   |
| Non-word              | \W        | &check;   |
| Digit                 | \d        | &check;   |
| Non-digit             | \D        | &check;   |
| Whitespace            | \s        | &check;   |
| Non-whitespace        | \S        | &check;   |
| Unicode category      | \p{L}     | &cross;   |
| Non-unicode category  | \p{L}     | &cross;   |
| Unicode script        | \p{Han}   | &cross;   |
| Non-unicode script    | \P{Han}   | &cross;   |
</center>

### Anchors
<center>

| Feature               | Example   | Supported |
|:---------------------:|:---------:|:---------:|
| Beginning             | ^         | &check;   |
| End                   | $         | &check;   |
| Word boundary         | \b        | &cross;   |
| Non-word boundary     | \B        | &cross;   |
</center>

### Groups & References
<center>

| Feature               | Example       | Supported |
|:---------------------:|:-------------:|:---------:|
| Capturing group       | (ABC)         | &check;   |
| Named capturing group | (?<name>ABC)  | &check;   |
| Numeric reference     | \1            | &cross;   |
| Non-capturing group   | (?:ABC)       | &check;   |
</center>

### Lookaround
<center>

| Feature               | Example       | Supported |
|:---------------------:|:-------------:|:---------:|
| Positive lookahead    | (?=ABC)       | &cross;   |
| Negative lookahead    | (?!ABC)       | &cross;   |
| Positive lookbehind   | (?<=ABC)      | &cross;   |
| Negative lookbehind   | (?<!ABC)      | &cross;   |
</center>

### Qualifiers & Alternation
<center>

| Feature               | Example       | Supported |
|:---------------------:|:-------------:|:---------:|
| Plus                  | +             | &check;   |
| Star                  | *             | &cross;   |
| Quantifier            | {1,3}         | &check;   |
| Optional              | ?             | &check;   |
| Lazy                  | ?             | &cross;   |
| Alternation           | \|            | &check;   |
</center>
