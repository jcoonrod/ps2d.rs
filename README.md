Redox won't boot on most modern x86 hardware due to limited drivers. In my case, I need a more relaxed ps2d driver

**Step 2: Route Your Recipe to the Patch** 
Now that your local driver repo is patched, configure your main Redox build directory to compile it:Open your main redox/ directory.Ensure you have the recipes/ps2d/recipe.toml file available.Edit the [source] section of that file to point directly to your modified folder path:toml[source]
path = "/absolute/path/to/your/cloned/drivers/input/ps2d"
Use code with caution.
**Step 3: Rebuild the Driver and Flash** 
Clean the target packages and rebuild your server live image by running these commands from your root redox/ folder:bash# Erase old pre-compiled ps2d binaries
make r.ps2d clean

# Compile your new patched code from source
make r.ps2d

# Re-bake everything into your live server image
make image
Use code with caution.
