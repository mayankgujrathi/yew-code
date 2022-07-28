lua << EOF
-- nvim_lsp.rust_analyzer.setup({
--     -- on_attach=on_attach,
--     settings = {
--         ["rust-analyzer"] = {
--             assist = {
--                 importGranularity = "module",
--                 importPrefix = "by_self",
--             },
--             cargo = {
--                 loadOutDirsFromCheck = true
--             },
--             procMacro = {
--                 enable = true
--             },
--         }
--     }
-- })
require'lspconfig'.rust_analyzer.setup{
    cmd = { "rust-analyzer" },
    filetypes = { "rust" },
    -- root_dir = root_pattern("Cargo.toml", "rust-project.json"),
    settings = {
        ["rust-analyzer"] = {
            assist = {
                importGranularity = "module",
                importPrefix = "by_self",
                },
            cargo = {
                loadOutDirsFromCheck = true
                },
            procMacro = {
            enable = true
            },
        }
    },
    on_attach=require'cmp'.on_attach
}
EOF

:set makeprg=cargo
:set errorformat=\ -->\ %f:%l:%c
