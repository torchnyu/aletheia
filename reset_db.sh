dropdb aletheia
createdb aletheia
diesel migration run
psql $DATABASE_URL -f seed.sql
