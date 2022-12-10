echo ${INPUT_7Z_PASSWORD}

echo "Find all input.txt and encrypt them"
set -e
for f in `find ./ -name "input.txt"`
do
    DIR="$(dirname "${f}")" ; 
    cd ${DIR}
    7z a input.7z input.txt -p${INPUT_7Z_PASSWORD}
    cd ..

done