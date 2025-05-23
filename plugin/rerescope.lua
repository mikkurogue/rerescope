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

-- Run the binary in a floating window to display the TUI and capture the selected file
local function run_rerescope()
    local buf = vim.api.nvim_create_buf(false, true) -- Create a scratch buffer
    local width = math.floor(vim.o.columns * 0.8)
    local height = math.floor(vim.o.lines * 0.8)
    local opts = {
        relative = "editor",
        width = width,
        height = height,
        col = math.floor((vim.o.columns - width) / 2),
        row = math.floor((vim.o.lines - height) / 2),
        style = "minimal",
        border = "rounded",
    }

    local win = vim.api.nvim_open_win(buf, true, opts) -- Open a floating window

    vim.fn.termopen("rerescope", {
        on_exit = function(_, exit_code)
            vim.api.nvim_win_close(win, true) -- Close the floating window
            if exit_code == 0 then
                local lines = vim.api.nvim_buf_get_lines(buf, 0, -1, false)
                for _, line in ipairs(lines) do
                    if line and line ~= "" then
                        local absolute_path = vim.fn.fnamemodify(line, ":p") -- Convert to absolute path
                        if vim.fn.filereadable(absolute_path) == 1 then
                            vim.cmd("edit " .. absolute_path) -- Open the file in a new buffer
                        end
                        break
                    end
                end
            end
            vim.api.nvim_buf_delete(buf, { force = true }) -- Clean up the buffer
        end,
    })

    vim.cmd("startinsert") -- Start in insert mode for the terminal
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