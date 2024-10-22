# !(ls >> x.py)
__xonsh__.subproc_captured_object('ls', '>>', 'x.py')

# !(ls | grep wakka > x.py)
__xonsh__.subproc_captured_object('ls', '|', 'grep', 'wakka', '>', 'x.py')

# !(ls | grep wakka >> x.py)
__xonsh__.subproc_captured_object('ls', '|', 'grep', 'wakka', '>>', 'x.py')

# $(ls > x.py)
__xonsh__.subproc_captured('ls', '>', 'x.py')

# $(ls >> x.py)
__xonsh__.subproc_captured('ls', '>>', 'x.py')

# $(ls | grep wakka > x.py)
__xonsh__.subproc_captured('ls', '|', 'grep', 'wakka', '>', 'x.py')

# $(ls | grep wakka >> x.py)
__xonsh__.subproc_captured('ls', '|', 'grep', 'wakka', '>>', 'x.py')
