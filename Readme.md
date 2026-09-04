

docker compose exec db pg_dump -U POSTGRES_USER DB_NAME > backup.sql

mkdir -p ./db_data
