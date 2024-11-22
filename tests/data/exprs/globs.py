# `hello`
__xonsh__.regex_literal('hello')

# $(ls `#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal('#[Ff]+i*LE'), '-l')

# $(ls r`[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal(r'[Ff]+i*LE'), '-l')

# $(ls r`#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal(r'#[Ff]+i*LE'), '-l')

# $(ls g`[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal('g`[Ff]+i*LE'), '-l')

# $(ls g`#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal('g`#[Ff]+i*LE'), '-l')

# $(ls @foo`[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal('@foo`[Ff]+i*LE'), '-l')

# print(@foo`.*`)
print(__xonsh__.regex_literal('@foo`.*'))

# $(ls @foo`#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal('@foo`#[Ff]+i*LE'), '-l')

# print(`#.*`)
print(__xonsh__.regex_literal('#.*'))

# $(ls `[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.regex_literal('[Ff]+i*LE'), '-l')
