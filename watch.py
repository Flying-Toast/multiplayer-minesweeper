#!/usr/bin/python3 -B
def watch(on_change, delay_secs, watch_dirs=[], watch_files=[]):
    import time
    import os
    from glob import glob
    from pathlib import Path
    prev_checksums = []
    while True:
        files = []
        for watchdir in watch_dirs:
            files += sorted(list(filter(lambda i: os.path.isfile(i), glob(f"{watchdir}/**", recursive=True))))
        files += watch_files
        curr_checksums = list(map(lambda i: f"{i}: {hash(Path(i).read_text())}", files))
        if curr_checksums != prev_checksums:
            on_change()
        prev_checksums = curr_checksums
        time.sleep(delay_secs)

import subprocess

def on_change():
    if on_change.running_process is not None:
        on_change.running_process.terminate()
        on_change.running_process = None
    print("===== BUILDING... =====")
    result = subprocess.run(["cargo", "build"])
    if result.returncode == 0:
        on_change.running_process = subprocess.Popen(["cargo", "run"])
    else:
        print("===== ERROR RUNNING `cargo build` =====")

on_change.running_process = None

if __name__ == "__main__":
    watch(
        on_change, 1,
        watch_dirs=["src"],
        watch_files=["Cargo.toml"]
    )
