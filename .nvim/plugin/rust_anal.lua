vim.lsp.config("rust_analyzer", {
    settings = {
        ["rust-analyzer"] = {
            check = {
                command = "clippy",
                extraArgs = {
                    "--",
                    "-W",
                    "clippy::pedantic",
                },
            },
        },
    },
})

vim.lsp.enable({ "rust_analyzer" })
