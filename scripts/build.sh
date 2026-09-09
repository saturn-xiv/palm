#!/bin/bash

set -e

source /etc/os-release

export WORK_DIR=$PWD
export PACKAGE=palm-$VERSION_CODENAME-$(git describe --tags --always --dirty)
export TARGET_DIR=$WORK_DIR/tmp

# ---------------------------------------------------------

function build_dashboard() {
    cd $WORK_DIR/${1}/dashboard/
    if [ ! -d node_modules ]
    then
        npm install --silent
    fi
    npm run build -- --outDir ${TARGET_DIR}/${PACKAGE}/${1}/dashboard --logLevel silent
}

function build_wisteria_backend() {
    cd $WORK_DIR/

    local target="$1-unknown-linux-gnu"
    echo "building wisteria for $target"
    cargo build --release --quiet --target $target

    mkdir -p ${TARGET_DIR}/${PACKAGE}/bin/${1}
    cp ${WORK_DIR}/target/${target}/release/wisteria ${TARGET_DIR}/${PACKAGE}/bin/${1}/
}

function build_wisteria_assets() {
    cd $WORK_DIR/wisteria/
    if [ ! -d node_modules ]
    then
        npm install --silent
    fi

    local target=${TARGET_DIR}/${PACKAGE}/${1}
    mkdir -p $target

    local -a items=(
        "@popperjs/core/dist/umd"
        "bootstrap/dist"
        "@tabler/core/dist"
        "@material/web"
        "bulma/css/bulma.min.css"
        "dayjs/dayjs.min.js"
        "dayjs/locale"
        "dayjs/plugin"
        "@fortawesome/fontawesome-free/css"
        "@fortawesome/fontawesome-free/js"
        "@fortawesome/fontawesome-free/sprites-full"
        "@fortawesome/fontawesome-free/svgs-full"
        "@fortawesome/fontawesome-free/webfonts"
        "@picocss/pico/css"
        "foundation-sites/dist"
    )
    for it in "${items[@]}"
    do
        local d=$(dirname $target/node_modules/$it)
        mkdir -p $d
        cp -r node_modules/$it $d/
    done

    cp -r db assets $target/
}

function build_marigold() {
    cd $WORK_DIR/marigold/
    mvn --quiet clean
    mvn --quiet package -Dmaven.test.skip=true

    local target=${TARGET_DIR}/${PACKAGE}/marigold
    mkdir -p $target
    cp target/marigold-*.jar README.md $target/
}

function generate_etc() {
    local target=${TARGET_DIR}/${PACKAGE}/etc
    mkdir -p $target/systemd $target/nginx

    cat <<EOF > $target/systemd/loquat.service
[Unit]
Description=A cryptographic rpc service(by Google Tink).
Documentation=https://github.com/saturn-xiv/palm/tree/main/loquat
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=nobody
Group=nogroup
ExecStart=/usr/local/bin/loquat rpc -p 10011
WorkingDirectory=/var/lib/loquat
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    cat <<EOF > $target/dahlia.toml
[postgresql]
host = '127.0.0.1'
port = 5432
user = 'www'
password = 'change-me'
db-name = 'dahlia_dev'

[rabbitmq]
host = '127.0.0.1'
port = 5672
user = 'www'
password = 'change-me'
virtual-host = 'dahlia.dev'
EOF
    cat <<EOF > $target/systemd/dahlia.service
[Unit]
Description=RBAC services.
Documentation=https://github.com/saturn-xiv/palm/tree/main/dahlia
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=nobody
Group=nogroup
ExecStart=source /opt/python3/bin/activate && dahlia -p 11002
WorkingDirectory=/var/lib/dahlia
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    cat <<EOF > $target/marigold.yaml
EOF
    cat <<EOF > $target/systemd/marigold.service
[Unit]
Description=WechatPay services.
Documentation=https://github.com/saturn-xiv/palm/tree/main/marigold
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=nobody
Group=nogroup
ExecStart=/opt/jdk/bin/java -jar marigold-2026.7.28.jar --spring.config.name=production
WorkingDirectory=/var/lib/marigold
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    cat <<EOF > $target/wisteria.toml
cookie-key = "openssl rand -base64 128"

[postgresql]
user = "www"
password = "change-me"
db-name = "wisteria_dev"

[redis]

[rabbitmq]
user = "www"
password = "change-me"
virtual-host = "wisteria.dev"

[opensearch]
namespace = "wisteria.dev"

[minio]
endpoint = "https://assets.change-me.org"
access-key = ""
secret-key = ""
namespace = "wisteria.dev"

[loquat]
port = 11001

[dahlia]
port = 11002

[marigold]
port = 11003

[lavender]
jobs-dir = "/var/lib/lavender/jobs"
work-dir = "/var/lib/lavender/cache"
bcc = []
EOF
    cat <<EOF > $target/systemd/wisteria.service
[Unit]
Description=An online education solution.
Documentation=https://github.com/saturn-xiv/palm/tree/main/wisteria
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=ubuntu
Group=ubuntu
ExecStart=/usr/local/bin/wisteria http -p 8080
WorkingDirectory=/var/lib/wisteria
Restart=always

Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
    cat <<EOF > $target/nginx/wisteria.conf
server {
    listen 80;
    server_name www.change-me.org;
    charset utf-8;

    location / {
        proxy_pass http://localhost:8080;

        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;

        client_max_body_size 512M;
    }

    location /my/ {
        alias /var/lib/wisteria/dashboard/;
        index index.html;
        try_files \$uri \$uri/ /my/index.html;
    }
}
EOF

}

# ---------------------------------------------------------

if [ -f ${TARGET_DIR}/${PACKAGE}.md5 ]
then
    echo "release $PACKAGE already exists."
    exit 1
fi

if [ -f ${TARGET_DIR}/${PACKAGE}.tar.xz ]
then
    rm ${TARGET_DIR}/${PACKAGE}.tar.xz
fi

if [ -d ${TARGET_DIR}/${PACKAGE} ]
then
    rm -r ${TARGET_DIR}/${PACKAGE}
fi

declare -a targets=("x86_64" "aarch64" "riscv64gc")
for t in "${targets[@]}"; do
    build_wisteria_backend $t
done

build_dashboard wisteria
build_wisteria_assets
build_marigold

cd $WORK_DIR/loquat/
bash build.sh

generate_etc

XZ_OPT=-9 tar -cJf ${TARGET_DIR}/${PACKAGE}.tar.xz --remove-files -C ${TARGET_DIR}/${PACKAGE} .
md5sum ${TARGET_DIR}/${PACKAGE}.tar.xz > ${TARGET_DIR}/${PACKAGE}.md5

echo "done(${PACKAGE}.tar.xz)."
exit 0
