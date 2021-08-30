#!/bin/bash

set -e

# ---------------------------------------------------------

echo "generate diesel schema..."
export DATABASE_URL="postgres://postgres@127.0.0.1:5432/demo"
diesel print-schema -o schema_migrations > src/orm/postgresql/schema.rs
diesel print-schema -o settings > src/settings/schema.rs
diesel print-schema -o locales > src/i18n/schema.rs

diesel print-schema -o logs users groups_users groups roles_users roles_groups roles_relations roles operations resources policies > src/auth/models/schema.rs
# FIXME
# diesel print-schema -o tags notifications attachments > src/plugins/nut/models/schema.rs
diesel print-schema -o crawler_sites crawler_logs > src/plugins/crawler/models/schema.rs
diesel print-schema -o sms_logs > src/plugins/twilio/models/schema.rs

# declare -A databases=( ["postgresql"]="postgres://postgres@127.0.0.1:5432/demo" ["mysql"]="mysql://root@127.0.0.1:3306/demo" ["sqlite"]="tmp/db")
#  for k in "${!databases[@]}"
    # export DATABASE_URL=${databases[$k]}
# do 

# done

# ---------------------------------------------------------

declare -a languages=(
    "php"
    "python"
    "ruby"
    "cpp"
)


for l in "${languages[@]}"
do
    if [ -d tmp/$l ]
    then
        rm -rv tmp/$l
    fi
    mkdir -pv tmp/$l
    protoc -I proto -I /usr/include/google/protobuf/ --${l}_out=tmp/$l --grpc_out=tmp/$l --plugin=protoc-gen-grpc=`which grpc_${l}_plugin` proto/*.proto
done


# ---------------------------------------------------------

echo "format code"
cargo fmt

# ---------------------------------------------------------

echo 'done.'
exit 0
