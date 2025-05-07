-- rerescope.lua: A Neovim plugin for integrating with the rerescope binary

-- Clone the repository, install the binary, and clean up
local function install_rerescope()
    local temp_dir = vim.fn.tempname()
    os.execute("git clone https://github.com/mikkurogue/rerescope " .. temp_dir)
    os.execute("cd " .. temp_dir .. " && cargo install --path .")
    os.execute("rm -rf " .. temp_dir)
end

-- Ensure the binary is installed and available
local function ensure_binary_installed()
    local handle = io.popen("which rerescope")
    local result = handle:read("*a")
    handle:close()

    if result == "" then
        print("Installing rerescope binary...")
        install_rerescope()
    end
end

-- Run the binary in a terminal to display the TUI
local function run_rerescope()
    vim.cmd("split | terminal rerescope")
end

-- Main function to integrate with Neovim
local function rerescope_find_files()
    ensure_binary_installed()
    run_rerescope()
end

-- Expose the command to Neovim
vim.api.nvim_create_user_command(
    "RerescopeFindFiles",
    rerescope_find_files,
    { desc = "Find and open files using rerescope" }
)