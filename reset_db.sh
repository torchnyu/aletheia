dropdb aletheia
createdb aletheia
diesel migration run
psql -f seed.sql -d aletheia
