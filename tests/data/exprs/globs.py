# $(ls `#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('`#[Ff]+i*LE`'), '-l')

# $(ls r`[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('r`[Ff]+i*LE`'), '-l')

# $(ls r`#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('r`#[Ff]+i*LE`'), '-l')

# $(ls g`[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('g`[Ff]+i*LE`'), '-l')

# $(ls g`#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('g`#[Ff]+i*LE`'), '-l')

# $(ls @foo`[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('@foo`[Ff]+i*LE`'), '-l')

# print(@foo`.*`)
print(__xonsh__.pathsearch('@foo`.*`'))

# $(ls @foo`#[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('@foo`#[Ff]+i*LE`'), '-l')

# print(`#.*`)
print(__xonsh__.pathsearch('`#.*`'))

# $(ls `[Ff]+i*LE` -l)
__xonsh__.subproc_captured('ls', __xonsh__.pathsearch('`[Ff]+i*LE`'), '-l')
