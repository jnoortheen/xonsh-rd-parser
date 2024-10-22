# $(ls | grep wakka)
__xonsh__.subproc_captured('ls', '|', 'grep', 'wakka')

# $(ls | grep wakka | grep jawaka)
__xonsh__.subproc_captured('ls', '|', 'grep', 'wakka', '|', 'grep', 'jawaka')

# ![ls me] and ![grep wakka]
__xonsh__.subproc_captured_hiddenobject('ls', 'me') and __xonsh__.subproc_captured_hiddenobject('grep', 'wakka')

# ![ls] and ![grep wakka] and ![grep jawaka]
__xonsh__.subproc_captured_hiddenobject('ls') and __xonsh__.subproc_captured_hiddenobject('grep', 'wakka') and __xonsh__.subproc_captured_hiddenobject('grep', 'jawaka')

# ![ls] && ![grep wakka]
__xonsh__.subproc_captured_hiddenobject('ls') and __xonsh__.subproc_captured_hiddenobject('grep', 'wakka')

# ![ls] && ![grep wakka] && ![grep jawaka]
__xonsh__.subproc_captured_hiddenobject('ls') and __xonsh__.subproc_captured_hiddenobject('grep', 'wakka') and __xonsh__.subproc_captured_hiddenobject('grep', 'jawaka')

# ![ls] or ![grep wakka]
__xonsh__.subproc_captured_hiddenobject('ls') or __xonsh__.subproc_captured_hiddenobject('grep', 'wakka')

# ![ls] or ![grep wakka] or ![grep jawaka]
__xonsh__.subproc_captured_hiddenobject('ls') or __xonsh__.subproc_captured_hiddenobject('grep', 'wakka') or __xonsh__.subproc_captured_hiddenobject('grep', 'jawaka')

# ![ls] || ![grep wakka]
__xonsh__.subproc_captured_hiddenobject('ls') or __xonsh__.subproc_captured_hiddenobject('grep', 'wakka')

# ![ls] || ![grep wakka] || ![grep jawaka]
__xonsh__.subproc_captured_hiddenobject('ls') or __xonsh__.subproc_captured_hiddenobject('grep', 'wakka') or __xonsh__.subproc_captured_hiddenobject('grep', 'jawaka')
