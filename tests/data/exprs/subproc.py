# $(cmd sub-cmd --opt)
__xonsh__.subproc_captured('cmd', 'sub-cmd', '--opt')

# $[cmd sub-cmd --opt]
__xonsh__.subproc_uncaptured('cmd', 'sub-cmd', '--opt')

# ![cmd sub-cmd --opt]
__xonsh__.subproc_captured_hiddenobject('cmd', 'sub-cmd', '--opt')

# !(cmd sub-cmd --opt)
__xonsh__.subproc_captured_object('cmd', 'sub-cmd', '--opt')

# ![git commit -am "wakka"]
__xonsh__.subproc_captured_hiddenobject('git', 'commit', '-am', '"wakka"')

# ![git commit -am "wakka jawaka"]
__xonsh__.subproc_captured_hiddenobject('git', 'commit', '-am', '"wakka jawaka"')

# ![ls "wakka jawaka baraka"]
__xonsh__.subproc_captured_hiddenobject('ls', '"wakka jawaka baraka"')

# $[git commit -am "wakka"]
__xonsh__.subproc_uncaptured('git', 'commit', '-am', '"wakka"')

# $[git commit -am "wakka jawaka"]
__xonsh__.subproc_uncaptured('git', 'commit', '-am', '"wakka jawaka"')

# $[ls "wakka jawaka baraka"]
__xonsh__.subproc_uncaptured('ls', '"wakka jawaka baraka"')

# ![echo ,]
__xonsh__.subproc_captured_hiddenobject('echo', ',')

# ![echo 1,2]
__xonsh__.subproc_captured_hiddenobject('echo', '1,2')

# !(echo '$foo')
__xonsh__.subproc_captured_object('echo', "'$foo'")

# !(echo r'$foo')
__xonsh__.subproc_captured_object('echo', "r'$foo'")
