FROM grafana/grafana:10.0.3
USER grafana

ARG POSTGRES_USER="postgres"
ARG POSTGRES_PASSWORD="postgres"
ARG POSTGRES_DB="postgres"

ENV POSTGRES_USER=${POSTGRES_USER}
ENV POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
ENV POSTGRES_DB=${POSTGRES_DB}

COPY --chown=grafana:grafana ./grafana/datasources/ /etc/grafana/provisioning/datasources/
COPY --chown=grafana:grafana ./grafana/dashboards/ /etc/grafana/provisioning/dashboards/
COPY --chown=grafana:grafana ./env_injection.sh /env_injection.sh


RUN sh -c "find /etc/grafana/provisioning/datasources/ -type f -exec sh -c \"source /env_injection.sh {} {}\" \;"
