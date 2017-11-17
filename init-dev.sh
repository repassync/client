#! /bin/sh

BASEDIR=$(readlink -f $(dirname $0))
DATADIR=$BASEDIR/data

echo "Compile resource…"
cd $DATADIR
glib-compile-resources repassync.gresource.xml

echo "Generate the path to data directory…"
echo "\"$DATADIR\"" > $BASEDIR/src/datadir.in
