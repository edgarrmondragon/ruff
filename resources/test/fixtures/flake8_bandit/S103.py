from os import chmod
import stat

keyfile = "foo"

chmod("/etc/passwd", 0o227)  # Error
chmod("/etc/passwd", 0o7)  # Error
chmod("/etc/passwd", 0o664)  # OK
chmod("/etc/passwd", 0o777)  # Error
chmod("/etc/passwd", 0o770)  # Error
chmod("/etc/passwd", 0o776)  # Error
chmod("/etc/passwd", 0o760)  # OK
chmod("~/.bashrc", 511)  # Error
chmod("/etc/hosts", 0o777)  # Error
chmod("/tmp/oh_hai", 0x1FF)  # Error
chmod("/etc/passwd", stat.S_IRWXU)  # OK
chmod(keyfile, 0o777)  # Error
chmod(keyfile, 0o7 | 0o70 | 0o700)  # Error
chmod(keyfile, stat.S_IRWXO | stat.S_IRWXG | stat.S_IRWXU)  # Error
chmod("~/hidden_exec", stat.S_IXGRP)  # Error
chmod("~/hidden_exec", stat.S_IXOTH)  # OK
chmod("/etc/passwd", stat.S_IWOTH)  # Error
