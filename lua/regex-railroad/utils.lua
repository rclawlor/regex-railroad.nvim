local function locate_binary()
    -- TODO: make function robust
    return "./target/debug/regex-railroad"
end

-- http://lua-users.org/wiki/FileInputOutput

-- see if the file exists
local function file_exists(file)
    local f = io.open(file, "rb")
    if f then f:close() end
    return f ~= nil
end

-- get all lines from a file, returns an empty 
-- list/table if the file does not exist
local function lines_from_file(file)
    if not file_exists(file) then return {} end
    local lines = {}
    for line in io.lines(file) do
        lines[#lines + 1] = line
    end
    return lines
end

return {
    locate_binary = locate_binary,
    lines_from_file = lines_from_file,
}

