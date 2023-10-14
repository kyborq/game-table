![image](https://github.com/kyborq/game-table/assets/52314985/76487375-1b46-4d26-b440-bd19fff282af)

# GamePulse

Store and retrieve information about your player state and score

## Backend

0. Install sqlx tools

```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

1. Database creation from DATABASE_URL

```
sqlx database create
```

2. Running migration

Actually there is two ways of doing this

```
sqlx migrate run
```

And other way

```
sqlx::migrate!("./migrations").run(&pool).await;
```
