#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 PORT"
    exit 1
fi

export WORK_DIR="$PWD/mnt"
export CODE="palm-autumn"
export NAME="$CODE-$1"

if [ ! -f $WORK_DIR/authorized_keys ]
then
    echo "Couldn't found authorized_keys file"
    exit 1
fi

cat << EOF > $WORK_DIR/start.sh
#!/bin/bash

set -e

mkdir -p \$HOME/.ssh
chmod 700 \$HOME/.ssh
cp /mnt/authorized_keys \$HOME/.ssh/
chmod 644 \$HOME/.ssh/authorized_keys

sudo /usr/sbin/sshd -D -e -4 -p $1
EOF
chmod +x $WORK_DIR/start.sh

if [ "$(docker inspect -f '{{.State.Status}}' $NAME)" = "running" ]; then
    echo "container $NAME is active!"
    exit 0
fi

if [ -n "$(docker ps -a -q -f name="^${NAME}$")" ] ; then
    docker start $NAME
    exit 0
fi

docker run -d --name $NAME --hostname=autumn --network host -v $WORK_DIR:/mnt:z $CODE /mnt/start.sh

exit 0
