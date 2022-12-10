# !/bin/bash
# To run on command line, do 
#   export INPUT_7Z_PASSWORD=pass1234
echo ${INPUT_7Z_PASSWORD}

CWD="$(pwd)"
echo " "
echo Current path = "$CWD"


echo "Find all input.txt and encrypt them"
for f in `find ./ -name "input.txt"`
do
    DIR="$(dirname "${f}")" ; 
    cd ${DIR}
    7z a input.7z input.txt -p${INPUT_7Z_PASSWORD}
    cd ..

done