def print_memstats() -> bool:
    import sys

    import psutil

    mega_bytes = 2**20
    print("Memory stats:")
    process = psutil.Process()
    meminfo = process.memory_info()
    res = {}
    res["rss"] = meminfo.rss / mega_bytes
    res["vms"] = meminfo.vms / mega_bytes
    if sys.platform == "win32":
        res["maxrss"] = meminfo.peak_wset / mega_bytes
    else:
        # See https://stackoverflow.com/questions/938733/total-memory-used-by-python-process
        import resource  # Since it doesn't exist on Windows.

        rusage = resource.getrusage(resource.RUSAGE_SELF)
        if sys.platform == "darwin":
            factor = 1
        else:
            factor = 1024  # Linux
        res["maxrss"] = rusage.ru_maxrss * factor / mega_bytes
    for key, value in res.items():
        print(f"  {key:12.12s}: {value:10.0f} MiB")
    return True


def sm():
    """The minimum program uses around 16MB memory"""
    # Memory stats:
    #   rss         :         16 MiB
    #   vms         :     401345 MiB
    #   maxrss      :         16 MiB
    import ast

    tree = ast.parse("print(1)")
    print(ast.dump(tree))


def main():
    # Memory stats:
    #   rss         :         19 MiB
    #   vms         :     401474 MiB
    #   maxrss      :         19 MiB
    from xonsh_rd_parser import Parser

    src_txt = "print(1)"
    ast = Parser(src_txt).parse()
    print(f"ast: {ast}", type(ast))


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("--empty", action="store_true")
    args = parser.parse_args()
    if args.empty:
        sm()
    else:
        main()
    print_memstats()
