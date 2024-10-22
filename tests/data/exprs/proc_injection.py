# $[@$(which python)]
__xonsh__.subproc_uncaptured(*__xonsh__.subproc_captured_inject('which', 'python'))

# $[ls @$(dirname @$(which python))]
__xonsh__.subproc_uncaptured('ls', *__xonsh__.subproc_captured_inject('dirname', *__xonsh__.subproc_captured_inject('which', 'python')))

# ![a@$(echo 1 2)b]
__xonsh__.subproc_captured_hiddenobject(('a', *__xonsh__.subproc_captured_inject('echo', '1', '2'), 'b'))
