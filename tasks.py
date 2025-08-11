import contextlib
import shlex
import subprocess as sp
from pathlib import Path
from packaging import version

import tomlkit


class Color:
    HEADER = "\033[95m"
    OKBLUE = "\033[94m"
    OKGREEN = "\033[92m"
    WARNING = "\033[93m"
    FAIL = "\033[91m"
    BOLD = "\033[1m"
    UNDERLINE = "\033[4m"

    # reset
    RESET = "\033[0m"


def run(cmd, **kwargs):
    kwargs.setdefault("check", True)
    tokens = shlex.split(cmd)
    print(f"{Color.OKGREEN} $ {tokens}{Color.RESET}")
    return sp.run(tokens, **kwargs)


def _get_next_tag(current_tag: version.Version) -> str | None:
    matching_tags = run(
        'git tag --sort=creatordate -l "0.*"', capture_output=True, text=True
    ).stdout.splitlines()
    for tag in matching_tags:
        if version.parse(tag) > current_tag:
            return tag
    return None


@contextlib.contextmanager
def toml_file(path: str | Path):
    if isinstance(path, str):
        path = Path(path)
    toml_doc = tomlkit.loads(path.read_text())
    yield toml_doc
    path.write_text(tomlkit.dumps(toml_doc))


def _update_cargo_deps() -> tuple[str, str | None]:
    with toml_file("Cargo.toml") as cargo_file:
        dependencies = cargo_file["workspace"]["dependencies"]
        current_tag = dependencies["ruff_python_ast"]["tag"]
        next_tag = _get_next_tag(version.parse(current_tag))
        if next_tag is None:
            return current_tag, next_tag

        for dep, dep_data in dependencies.items():
            if dep.startswith("ruff_") and dep_data.get("tag") == current_tag:
                dependencies[dep]["tag"] = next_tag
    return current_tag, next_tag


def pull_ruff_crates():
    run("git fetch ruff-repo --tags", check=False)
    current, to = _update_cargo_deps()
    if to is None:
        print("No new version of ruff_python_parser found")
        return
    run(
        f"git format-patch {current}..{to} --output-directory=parser-patches -- crates/ruff_python_parser"
    )
