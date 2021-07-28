#!/bin/bash

set -e

# ---------------------------------------------------------

echo "generate diesel schema..."
export DATABASE_URL="postgres://postgres@127.0.0.1:5432/palm"
diesel print-schema -o schema_migrations > src/orm/postgresql/schema.rs
diesel print-schema -o settings > src/settings/schema.rs
diesel print-schema -o locales > src/i18n/schema.rs
diesel print-schema -o tags notifications attachments logs users groups_users groups roles_users roles_groups roles_relations roles operations resources policies > src/plugins/nut/models/schema.rs
diesel print-schema -o crawler_sites crawler_logs > src/plugins/crawler/models/schema.rs
diesel print-schema -o sms_logs > src/plugins/sms/models/schema.rs

# declare -A databases=( ["postgresql"]="postgres://postgres@127.0.0.1:5432/palm" ["mysql"]="mysql://root@127.0.0.1:3306/palm" ["sqlite"]="tmp/db")
#  for k in "${!databases[@]}"
    # export DATABASE_URL=${databases[$k]}
# do 

# done

# ---------------------------------------------------------

# declare -a services=(
#     "nut"
#     "lemon"
# )

# declare -a languages=(
#     "php"
#     "python"
# )

# for s in "${services[@]}"
# do
#     for l in "${languages[@]}"
#     do
#         mkdir -pv clients/$l/$s
#         protoc -I $s/proto -I $HOME/.local/include/google/protobuf --${l}_out=clients/$l/$s --grpc_out=clients/$l/$s --plugin=protoc-gen-grpc=`which grpc_${l}_plugin` $s/proto/*.proto
#     done
# done

# ---------------------------------------------------------

echo "format code"
cargo fmt

# ---------------------------------------------------------

echo 'done.'
exit 0
