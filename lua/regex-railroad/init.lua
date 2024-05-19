local regex_railroad = { }

function regex_railroad.setup(opts)
    opts = opts or {} 

    require("regex-railroad.config").set_defaults(opts.defaults)
end

return regex_railroad
