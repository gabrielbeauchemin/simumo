#!/bin/bash

# This script is used to start  the Simumo then the visualiser after
# This is copied in the build when the system is built
#   first argument: configuration file path
#   Second argument: vizualizer congfiguration path.
# Make sure that logger path of each config file are the same.
# note :: this might be replaced with a python script in the future
unamestr=`uname`
echo Entering virtual environnement.
# the if line will probably change. We wnat to check if os is windows.
if [[ "$unamestr" == 'MINGW64_NT-10.0' ]]; then
    source venv/Scripts/activate
else
    source venv/bin/activate
fi
python ./visualizer/Server.py $1

deactivate
