

docker compose exec db pg_dump -U POSTGRES_USER DB_NAME > backup.sql


cat backup.sql | docker compose exec -T db psql -U postgres -d test_db


mkdir -p ./db_data
