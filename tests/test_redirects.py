import pytest


@pytest.mark.parametrize(
    "case", ["", "o", "out", "1", "e", "err", "2", "a", "all", "&"]
)
def test_redirect_output(case, cmd):
    assert cmd(f'$[echo "test" {case}> test.txt]') == [
        "echo",
        "test",
        {f"{case}>": "test.txt"},
    ], f'$[echo "test" {case}> test.txt]'
    assert cmd(f'$[< input.txt echo "test" {case}> test.txt]') == [
        "echo",
        "test",
        {"<": "input.txt", f"{case}>": "test.txt"},
    ]
    assert cmd(f'$[echo "test" {case}> test.txt < input.txt]') == [
        "echo",
        "test",
        {"<": "input.txt", f"{case}>": "test.txt"},
    ]


@pytest.mark.parametrize(
    "r",
    [
        "e>o",
        "e>out",
        "err>o",
        "2>1",
        "e>1",
        "err>1",
        "2>out",
        "2>o",
        "err>&1",
        "e>&1",
        "2>&1",
    ],
)
@pytest.mark.parametrize("o", ["", "o", "out", "1"])
def test_redirect_error_to_output(r, o, cmd):
    assert cmd(f'$[echo "test" {r} {o}> test.txt]') == [
        "echo",
        "test",
        {f"{o}>".strip(): "test.txt", r.split(">")[0] + ">": r.split(">")[1]},
    ]
    assert cmd(f'$[< input.txt echo "test" {r} {o}> test.txt]') == [
        "echo",
        "test",
        {
            f"{o}>".strip(): "test.txt",
            r.split(">")[0] + ">": r.split(">")[1],
            "<": "input.txt",
        },
    ]
    assert cmd(f'$[echo "test" {r} {o}> test.txt < input.txt]') == [
        "echo",
        "test",
        {
            f"{o}>".strip(): "test.txt",
            r.split(">")[0] + ">": r.split(">")[1],
            "<": "input.txt",
        },
    ]


@pytest.mark.parametrize(
    "r",
    [
        "o>e",
        "o>err",
        "out>e",
        "1>2",
        "o>2",
        "out>2",
        "1>err",
        "1>e",
        "out>&2",
        "o>&2",
        "1>&2",
    ],
)
@pytest.mark.parametrize("e", ["e", "err", "2"])
def test_redirect_output_to_error(r, e, cmd):
    assert cmd(f'$[echo "test" {r} {e}> test.txt]') == [
        "echo",
        "test",
        {f"{e}>": "test.txt", r.split(">")[0] + ">": r.split(">")[1]},
    ]
    assert cmd(f'$[< input.txt echo "test" {r} {e}> test.txt]') == [
        "echo",
        "test",
        {
            f"{e}>": "test.txt",
            r.split(">")[0] + ">": r.split(">")[1],
            "<": "input.txt",
        },
    ]
    assert cmd(f'$[echo "test" {r} {e}> test.txt < input.txt]') == [
        "echo",
        "test",
        {
            f"{e}>": "test.txt",
            r.split(">")[0] + ">": r.split(">")[1],
            "<": "input.txt",
        },
    ]
