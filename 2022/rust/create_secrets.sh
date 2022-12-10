# !/bin/bash
# To run on command line, do 
#   export INPUT_7Z_PASSWORD=pass1234
#INPUT_7Z_PASSWORD=$1
#echo ${INPUT_7Z_PASSWORD}

CWD="$(pwd)"
echo " "
echo Current path = "$CWD"


echo "Find all input.txt and encrypt them"
for f in `find ./ -name "input.txt"`
do
    DIR="$(dirname "${f}")" ; 
    cd ${DIR}
    if test -f "input.7z"; then
        echo "${DIR}/input.7z exists."
    else
        7z a input.7z input.txt -p${INPUT_7Z_PASSWORD}
    fi
    cd ..

done