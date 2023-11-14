FROM postgres:16.1

ADD pg_stat_statements.sh /docker-entrypoint-initdb.d
ADD ./init /docker-entrypoint-initdb.d
