# Diesel setup

It is necessary to add create folder `migrations` and the file `diesel.toml`
manually

## Generate a new table

To generate a new table with name `bloodpressure` use `generate`, this
generates a new folder for the migration and two files `up.sql` and `down.sql`.
```shell
diesel migration generate bloodpressure
```

## Run migrations

To run the migrations, simply call `run`. This updates the file `src/schema.rs`.
(Or whatever was assigned for `file` in `diesel.toml`). All migrations, which
are not run until now will be updated.
```shell
diesel migration run
```

