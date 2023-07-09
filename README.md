![caneta-azul](https://user-images.githubusercontent.com/23015763/210018694-5e9c2308-f0e8-4c37-86bf-f3d8c75b1e7a.gif)

## Running locally

Modify environment variables
```
cp .env.example .env
```

[Install sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install)

[Install AWS cli](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)

Run local development containers, run migrations, create localstack s3 buckets
```
make dev
```

Run tests
```
cargo t
```