os.execute("devenv lsp --print-config  >/tmp/devenv.json 2>/dev/null")
local file = assert(io.open("/tmp/devenv.json", "r"))
local settings = vim.json.decode(file:read("a")) ---Yes, this is needed... (I no, I must pipe it to a file...)
file:close()

return {
    cmd = { "devenv", "lsp" },
    filetypes = { "nix" },
    root_markers = { "flake.nix", ".git" },
    settings = settings,
}
