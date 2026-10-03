local popen = assert(io.popen("tmp=$(mktemp); devenv lsp --print-config >$tmp 2>/dev/null && echo $tmp"))
local r = popen:read("*a")
print(r)
popen:close()
