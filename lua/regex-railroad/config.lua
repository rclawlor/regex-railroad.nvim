local utils = require('regex-railroad.utils')
local job = require('regex-railroad.job')

local config = {}

function config.set_defaults()
    config.attach = job.attach
    config.detach = job.detach
    config.locate_binary = utils.locate_binary
end

return config
