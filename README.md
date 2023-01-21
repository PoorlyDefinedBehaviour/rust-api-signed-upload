![caneta-azul](https://user-images.githubusercontent.com/23015763/210018694-5e9c2308-f0e8-4c37-86bf-f3d8c75b1e7a.gif)

## Running locally

Modify environment variables
```
cp .env.example .env
```

Run database
```
docker-compose up -d
```

[Install sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#install)

Run migrations
```
sqlx migrate run
```