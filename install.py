from sys import platform
from installers import win, linux, osx

if platform == "win32":
  win.install()
elif platform == "darwin":
  osx.install()
elif "linux" in platform:
    linux.install()
