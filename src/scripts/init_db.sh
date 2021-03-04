DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=myhealth}"
DB_PORT="${POSTGRES_PORT:=5432}"

docker run \
    --name teste-postgres \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p 5432:5432 \
    -d postgres \
      postgres -N 1000
sleep 2
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"