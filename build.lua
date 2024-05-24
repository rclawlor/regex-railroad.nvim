vim.schedule(
    function()
        local home_dir = os.getenv("XDG_CONFIG_HOME") or "~/.local/share"
        local install_dir = string.format("{}/lazy/regex-railroad", home_dir)
        local utils = require(string.format("{}/regex-railroad.utils", install_dir))
        utils.install_binary()
    end
)
