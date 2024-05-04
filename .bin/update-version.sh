#! /bin/bash

TO_VERSION=$1

function update() {
    from=$1
    to=$2
    echo "sed -e "s/\${VERSION}/${TO_VERSION}/g" -e "s/\${VERSION_DH}/${TO_VERSION//-/--}/g" $from > $to"
    sed -e "s/\${VERSION}/${TO_VERSION}/g" -e "s/\${VERSION_DH}/${TO_VERSION//-/--}/g" $from > $to
}

for i in $(find .templates -type f)
do
    update $i ${i#.templates/}
done
