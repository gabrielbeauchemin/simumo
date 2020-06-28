from distutils.core import setup

import setuptools

setup(
    name='simumap',
    version='0.1',
    description='minimal API for more extensible data fetching overpy/osmgraph libraries',
    author='Jeremie Bigras-Dunberry',
    author_email='Bigjerbd@gmail.com',
    packages=setuptools.find_packages(),
    url='https://github.com/BigJerBD/simumap',
    license='MIT License',
    platforms=['POSIX', 'Windows', 'Unix', 'MacOS'],
    keywords=['python', 'osmgraph', 'API', 'minimal'],
    classifiers=(
        "Programming Language :: Python :: 3.6",
        "Operating System :: OS Independent",
        "Topic :: Utilities"
    ),
)
