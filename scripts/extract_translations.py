import json
import os
import platform
import shutil

from packaging import version
from pathlib import Path


def extract_translations(minecraft_dir: Path, output_dir: Path):
    output_dir.mkdir(parents=True, exist_ok=True)

    assets_indexes_dir = minecraft_dir / "assets" / "indexes"
    latest_version = None

    for index in assets_indexes_dir.glob("*.json"):
        index_version = version.parse(index.stem)
        if latest_version is None or index_version > latest_version:
            latest_version = index_version

    assets_index = assets_indexes_dir / f"{latest_version}.json"
    files_count = 0
    with open(assets_index) as f:
        assets = json.load(f)
        for object, index in assets["objects"].items():
            if not object.startswith("minecraft/lang"):
                continue
            output_file_path = output_dir / object.replace("minecraft/lang/", "")
            object_file_path = (
                minecraft_dir / "assets" / "objects" / index["hash"][:2] / index["hash"]
            )
            shutil.copy(object_file_path, output_file_path)
            files_count += 1

    return latest_version, files_count


def default_minecraft_dir():
    """
    Returns the default Minecraft data directory, according to the current platform.
    """
    os_name = platform.system().lower()
    if "win" in os_name:
        return os.path.expandvars("%appdata%\\.minecraft")
    elif "darwin" in os_name:
        return os.path.expanduser("~/Library/Application Support/minecraft")
    else:
        return os.path.expanduser("~/.minecraft")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(
        description="Extracts JSON translation files from Minecraft to Panoptès."
    )
    parser.add_argument(
        "--minecraft-dir",
        dest="minecraft_dir",
        default=default_minecraft_dir(),
        help="The Minecraft data directory",
    )
    parser.add_argument(
        "--output-dir",
        dest="output_dir",
        default="translations",
        help="The output directory, where to place all JSON translation files",
    )

    args = parser.parse_args()
    version, files_count = extract_translations(
        Path(args.minecraft_dir).expanduser().absolute(),
        Path(args.output_dir).expanduser().absolute(),
    )
    print(f"Successfully extracted {files_count} translation files for {version}.")
