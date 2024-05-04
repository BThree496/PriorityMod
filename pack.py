import os
import shutil
import threading

# Define the compile command
compile_cmd = "cargo build -r --features \"{flag}\" --target-dir \"./tmp/{game_name}\""
# Define the targets array
targets = [
    ["Skyrim Priority AE", "x86_64-pc-windows-msvc", "skyrim_ae", "/Data/SKSE/Plugins"],
    ["Fallout Priority NG", "x86_64-pc-windows-msvc", "fallout_4", "/Data/F4SE/Plugins"]
]

# Create "./tmp" and "./out" folders
if not os.path.exists("./tmp"):
    os.makedirs("./tmp")
if not os.path.exists("./out"):
    os.makedirs("./out")

# Define a function to run the compile command


def compile_target(target):
    game_name, arch, flag, zip_nested_dir = target
    cmd = compile_cmd.format(game_name=game_name, flag=flag)
    os.system(cmd)


# Use multiple threads to compile targets in parallel
threads = []
for target in targets:
    thread = threading.Thread(target=compile_target, args=(target,))
    threads.append(thread)
    thread.start()

for thread in threads:
    thread.join()

# Define a function to process target files


def process_target(target):
    game_name = target[0]
    zip_nested_dir = target[3]
    out_dir = f"./out/{game_name}"
    tmp_dir = f"./tmp/{game_name}"

    # Create the output directory
    os.makedirs(f"{out_dir}{zip_nested_dir}", exist_ok=True)
    
    # Copy files
    shutil.copy(f"{tmp_dir}/release/FallrimPriority.dll",
                f"{out_dir}{zip_nested_dir}/FallrimPriority.dll")
    shutil.copy("./FallrimPriority.ini", f"{out_dir}{zip_nested_dir}/FallrimPriority.ini")

    # Compress the directory
    shutil.make_archive(out_dir, "zip", root_dir=out_dir)

    # Remove the directory
    shutil.rmtree(out_dir)


# Use multiple threads to process target files in parallel
threads = []
for target in targets:
    thread = threading.Thread(target=process_target, args=(target,))
    threads.append(thread)
    thread.start()

for thread in threads:
    thread.join()

# Delete "./tmp" folder
shutil.rmtree("./tmp")

print("Finished")
