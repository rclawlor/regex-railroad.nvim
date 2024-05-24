local M = {}

-- Variables
M.binary_location = string.format(
    "{}/lazy/regex-railroad/regex-railroad",
    os.getenv("XDG_CONFIG_HOME") or "~/.local/share"
)


return M
