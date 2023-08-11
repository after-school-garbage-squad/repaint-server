# Re:paint server

## setup

<details>
<summary>for dev</summary>

### rustup

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### docker

```sh
paru -S docker docker-compose --needed
```

### just

Make compatible task runner.

```sh
paru -S just --needed
```

</details>
<details>
<summary>for use</summary>

### docker

```sh
paru -S docker docker-compose --needed
```

### just

Make compatible task runner.

```sh
paru -S just --needed
```

</details>

## structure

### development environment

snapshot at Aug 11 2023.

- Rust v1.71.1
- Docker v24.0.5
- Just v1.14.0

### library

Only major libraries are listed.

| useage    | library                                    |
| --------- | ------------------------------------------ |
| framework | [axum](https://github.com/tokio-rs/axum)   |
| ORM       | [SeaORM](https://github.com/SeaQL/sea-orm) |
