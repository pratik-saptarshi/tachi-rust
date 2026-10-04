# PostgreSQL 16 to 18 local data migration

The local Compose database now uses PostgreSQL 18 and mounts the named volume
at `/var/lib/postgresql`, which is the supported volume root for PostgreSQL 18
and later. PostgreSQL 16 and 18 data directories are not interchangeable. Do
not point PostgreSQL 18 at a PostgreSQL 16 data directory or delete the old
volume as part of this migration.

Before changing the image or volume mount, while the PostgreSQL 16 container is
still running, save a logical backup from the repository root:

```sh
docker compose exec -T db sh -c 'pg_dump -Fc -U "$POSTGRES_USER" -d "$POSTGRES_DB"' > postgres16-backup.dump
```

Verify that the backup is non-empty and stored somewhere safe. Then stop the
old stack without removing its volumes, update the Compose file, and start the
PostgreSQL 18 database:

```sh
docker compose down
docker compose up -d db
docker compose exec db pg_isready -U "$POSTGRES_USER" -d "$POSTGRES_DB"
```

The existing named volume remains mounted at its new root; PostgreSQL 18 creates
its own cluster under that root while the old PostgreSQL 16 files remain
preserved in the volume. Restore the configured database into the new cluster:

```sh
docker compose exec -T db sh -c 'pg_restore --no-owner --no-privileges -U "$POSTGRES_USER" -d "$POSTGRES_DB"' < postgres16-backup.dump
```

Check the restored application data before resuming development. This procedure
copies the Compose-configured database; dump any additional databases
individually and recreate required roles separately. Keep the backup and
original volume until verification is complete. Do not run
`docker compose down -v` during this procedure.
