vim.lsp.config("rust_analyzer", {
  settings = {
    ["rust-analyzer"] = {
      cargo = { target = "wasm32-unknown-unknown" },
      check = { targets = { "x86_64-unknown-linux-gnu", "wasm32-unknown-unknown" } },
    },
  },
})
