# !/bin/bash
# To run on command line, do 
#   export INPUT_7Z_PASSWORD=pass1234
#INPUT_7Z_PASSWORD=$1
#echo ${INPUT_7Z_PASSWORD}

CWD="$(pwd)"
echo " "
echo Current path = "$CWD"

echo "Find all input.7z and extract them"
for f in `find ./ -name "input.7z"`
do
    DIR="$(dirname "${f}")"
    cd ${DIR}
    7z x input.7z -p${INPUT_7Z_PASSWORD}
    cd ..
done