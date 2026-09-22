#!/bin/bash

set -e

source /etc/os-release

if [[ "$ID" != "ubuntu" ]]
then
    echo "Unsupported system: $PRETTY_NAME"
    exit 1
fi

export WORK_DIR=$PWD
export PACKAGE_NAME=palm_"$(git describe --tags --always --dirty)+${VERSION_CODENAME}"
export TARGET_DIR=$WORK_DIR/tmp

# -----------------------------------------------------------------------------

function build_dashboard() {
    cd $WORK_DIR/${1}/dashboard/

    if [ ! -d node_modules ]
    then
        npm install --silent
    fi
    npm run build -- --logLevel silent
}


function build_wisteria_backend() {
    cd $WORK_DIR/

    local -a targets=("x86_64" "aarch64" "riscv64gc")
    for t in "${targets[@]}"; do
        local target="$t-unknown-linux-gnu"
        echo "building wisteria for $target"
        cargo build --release --quiet --target $target
    done
}

function build_wisteria_assets() {
    cd $WORK_DIR/wisteria/
    if [ ! -d node_modules ]
    then
        npm install --silent
    fi

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
        local d=$(dirname $1/node_modules/$it)
        mkdir -p $d
        cp -r node_modules/$it $d/
    done

    cp -r README.md db assets locales $1/
}

function build_dahlia() {
    local python_home=$WORK_DIR/tmp/python3
    if [ ! -d $python_home ]
    then
        python3 -m venv $python_home
    fi
    source $python_home/bin/activate
    pip install build

    cd $WORK_DIR/dahlia/
    if [ -d dist ]
    then
        rm -r dist
    fi
    python3 -m build
    deactivate
}

function build_marigold() {
    cd $WORK_DIR/marigold/
    mvn --quiet clean
    mvn --quiet package -Dmaven.test.skip=true
}

function generate_etc() {
    cat <<EOF > $1/loquat/rpc.service
[Unit]
Description=A cryptographic rpc service(by Google Tink).
Documentation=https://github.com/saturn-xiv/palm/tree/main/loquat
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
DynamicUser=yes
ExecStart=/usr/bin/loquat rpc -p 11011
WorkingDirectory=/var/lib/palm/loquat
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    cat <<EOF > $1/dahlia/config.toml
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
    cat <<EOF > $1/dahlia/rpc.service
[Unit]
Description=RBAC services.
Documentation=https://github.com/saturn-xiv/palm/tree/main/dahlia
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
DynamicUser=yes
ExecStart=/bin/bash -c "source /opt/python3/bin/activate && dahlia -p 11002"
WorkingDirectory=/var/lib/palm/dahlia
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    cat <<EOF > $1/marigold/production.yaml
server:
    port: 11003
spring:
    grpc:
    server:
        port: 11004
    datasource:
    url: jdbc:postgresql://localhost:5432/marigold_dev
    username: www
    password: "change-me"
    driver-class-name: org.postgresql.Driver
    hikari:
        maximum-pool-size: 10
        minimum-idle: 5
EOF
    cat <<EOF > $1/marigold/rpc.service
[Unit]
Description=WechatPay services.
Documentation=https://github.com/saturn-xiv/palm/tree/main/marigold
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
DynamicUser=yes
ExecStart=/opt/amazon-corretto-26.0.2.11.1-linux-aarch64/bin/java -jar marigold-$(git describe --tags --abbrev=7 --always | sed 's/-g/-/')*.jar --spring.config.name=production
WorkingDirectory=/var/lib/palm/marigold
Restart=always

[Install]
WantedBy=multi-user.target
EOF

    cat <<EOF > $1/wisteria/config.toml
cookie-key = "openssl rand -base64 128"

[postgresql]
host = "127.0.0.1"
port = 5432
user = "www"
password = "change-me"
db-name = "wisteria_dev"

[redis]
host = "127.0.0.1"
port = 6379

[rabbitmq]
host = "127.0.0.1"
port = 5672
user = "www"
password = "change-me"
virtual-host = "wisteria.dev"

[opensearch]
host = "http://127.0.0.1:9200"
namespace = "wisteria.dev"

[minio]
endpoint = "https://assets.change-me.org"
access-key = "change-me"
secret-key = "change-me"
namespace = "wisteria.dev"

[smtp]
host = "smtp.gmail.com"
port = 465
user = "change-me@gmail.com"
password = "change-me"

[loquat]
port = 11001

[dahlia]
port = 11002

[marigold]
port = 11003

[lavender]
jobs-dir = "/var/lib/palm/lavender/jobs"
working-dir = "/var/lib/palm/lavender/cache"
bcc = []
EOF
    cat <<EOF > $1/wisteria/http.service
[Unit]
Description=An online education solution.
Documentation=https://github.com/saturn-xiv/palm/tree/main/wisteria
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=ubuntu
Group=ubuntu
ExecStart=/usr/bin/wisteria http -p 11005
WorkingDirectory=/var/lib/wisteria
Restart=always

Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
    cat <<EOF > $1/wisteria/nginx.conf
server {
    listen 80;
    server_name www.change-me.org;
    charset utf-8;

    access_log /var/log/nginx/\$server_name.access.log;
    error_log /var/log/nginx/\$server_name.error.log warn;

    location / {
        proxy_pass http://localhost:11005;

        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;

        client_max_body_size 512M;
    }

    location /my/ {
        alias /var/lib/palm/wisteria/dashboard/;
        index index.html;
        try_files \$uri \$uri/ /my/index.html;
    }
}
EOF

}

function build_deb() {
    local package=${PACKAGE_NAME}_$1
    echo "building $package"

    cd ${TARGET_DIR}/
    mkdir DEBIAN
    cat <<EOF > DEBIAN/control
Package: palm
Version: $(git describe --tags --always --dirty)
Architecture: ${1}
Maintainer: "$(git log -1 --pretty=format:'%an')" <"$(git log -1 --pretty=format:'%ae')">
Depends: bash (>= 4.0)
Section: net
Priority: optional
Description: An open-source online education solution
    This package need jvm & python3 runtime.
EOF

    cd ${TARGET_DIR}/
    dpkg-deb --build $package ${package}.deb
    md5sum ${package}.deb > ${package}.md5
}

# -----------------------------------------------------------------------------


build_dashboard wisteria
build_dahlia
build_marigold

declare -a architectures=("amd64" "arm64" "riscv64")
for $a in "${architectures[@]}"; do
    target=${TARGET_DIR}/${PACKAGE_NAME}_$a

    mkdir -p $target/usr/share/palm/wisteria
    build_wisteria_assets $target/usr/share/palm/wisteria
    cp -r $WORK_DIR/wisteria/dashboard/dist $target/usr/share/palm/wisteria/dashboard

    cd $WORK_DIR/dahlia/
    mkdir -p $target/usr/share/palm/dahlia
    cp README.md dist/dahlia-*-py3-none-any.whl $target/usr/share/palm/wisteria/dahlia/

    cd $WORK_DIR/loquat/
    mkdir -p $target/usr/share/palm/loquat
    cp README.md $target/usr/share/palm/loquat/

    cd $WORK_DIR/marigold/
    mkdir -p $target/usr/share/palm/marigold
    cp target/marigold-*.jar README.md $target/usr/share/palm/marigold/

    generate_etc $target/usr/share/palm
done


cd $WORK_DIR/loquat/
bash build.sh
cp build/x86_64/loquat $${TARGET_DIR}/${PACKAGE_NAME}_amd64/usr/bin/
cp build/aarch64/loquat $${TARGET_DIR}/${PACKAGE_NAME}_arm64/usr/bin/
cp build/riscv64/loquat $${TARGET_DIR}/${PACKAGE_NAME}_riscv64/usr/bin/

build_wisteria_backend
cd $WORK_DIR/wisteria/target/
cp x86_64-unknown-linux-gnu/release/wisteria $${TARGET_DIR}/${PACKAGE_NAME}_amd64/usr/bin/
cp aarch64-unknown-linux-gnu/release/wisteria $${TARGET_DIR}/${PACKAGE_NAME}_arm64/usr/bin/
cp riscv64gc-unknown-linux-gnu/release/wisteria $${TARGET_DIR}/${PACKAGE_NAME}_riscv64/usr/bin/

for $a in "${architectures[@]}"; do
    build_deb $1
done

# -----------------------------------------------------------------------------

echo "done(${PACKAGE_NAME})."
exit 0
