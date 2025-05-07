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

-- Run the binary and get file suggestions
local function run_rerescope()
    local cmd = "rerescope " 
    local handle = io.popen(cmd)
    if not handle then
        print("Failed to execute rerescope binary.")
        return {}
    end

    local result = handle:read("*a")
    handle:close()

    if result == "" then
        print("No output from rerescope.")
        return {}
    end

    return vim.split(result, "\n", { trimempty = true })
end

-- Open a selected file in Neovim
local function open_file(file_path)
    vim.cmd("edit " .. file_path)
end

-- Main function to integrate with Neovim
local function rerescope_find_files()
    ensure_binary_installed()

    local args = {"--find"} -- Example argument, adjust as needed
    local files = run_rerescope()

    if #files == 0 then
        print("No files found.")
        return
    end

    -- Display files and let the user pick one
    print("Select a file to open:")
    for i, file in ipairs(files) do
        print(i .. ": " .. file)
    end

    local choice = tonumber(vim.fn.input("Enter the number of the file to open: "))
    if choice and choice > 0 and choice <= #files then
        open_file(files[choice])
    else
        print("Invalid choice.")
    end
end

-- Expose the command to Neovim
vim.api.nvim_create_user_command(
    "RerescopeFindFiles",
    rerescope_find_files,
    { desc = "Find and open files using rerescope" }
)