
from distutils.core import setup

setup(
    name = 'spritepack',
    install_requires = [
        'pillow'
    ]
    entry_points = {
        'console_scripts' : ['spritepack = spritepack.cli:main']
    }
)
