import pytest


@pytest.mark.parametrize("case", ["", "o", "out", "1"])
def test_redirect_output(case, exec_code):
    assert exec_code(f'$[echo "test" {case}> test.txt]')
    assert exec_code(f'$[< input.txt echo "test" {case}> test.txt]')
    assert exec_code(f'$[echo "test" {case}> test.txt < input.txt]')


@pytest.mark.parametrize("case", ["e", "err", "2"])
def test_redirect_error(case, exec_code):
    assert exec_code(f'$[echo "test" {case}> test.txt]')
    assert exec_code(f'$[< input.txt echo "test" {case}> test.txt]')
    assert exec_code(f'$[echo "test" {case}> test.txt < input.txt]')


@pytest.mark.parametrize("case", ["a", "all", "&"])
def test_redirect_all(case, exec_code):
    assert exec_code(f'$[echo "test" {case}> test.txt]')
    assert exec_code(f'$[< input.txt echo "test" {case}> test.txt]')
    assert exec_code(f'$[echo "test" {case}> test.txt < input.txt]')


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
def test_redirect_error_to_output(r, o, exec_code):
    assert exec_code(f'$[echo "test" {r} {o}> test.txt]')
    assert exec_code(f'$[< input.txt echo "test" {r} {o}> test.txt]')
    assert exec_code(f'$[echo "test" {r} {o}> test.txt < input.txt]')


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
def test_redirect_output_to_error(r, e, exec_code):
    assert exec_code(f'$[echo "test" {r} {e}> test.txt]')
    assert exec_code(f'$[< input.txt echo "test" {r} {e}> test.txt]')
    assert exec_code(f'$[echo "test" {r} {e}> test.txt < input.txt]')
