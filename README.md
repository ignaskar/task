# Task

## 1. Running the solution

This application requires Rust version 1.74 or greater. In order to run this solution, please make sure that you have `docker`, `psql` and `diesel` CLI tools installed.

### 1.1 Running the database

Simply execute `init_db.sh` under `./backend/scripts/` - this will take care of spinning up a new Postgres instance in Docker
and applying migrations.

### 1.2 Running the server

Plain-old `cargo run` in `./backend/` will suffice here.

## 2. Endpoints

Here are some example requests that you can send to test out the solution.

`POST http://localhost:8000/auth/register`
```json
{
  "name": "ignas karpusenkovas",
  "email": "ignas.karpusenkovas@gmail.com",
  "password": "t0B7eAEPQEjY00w4MFNUQ1mPk1"
}
```

`POST http://localhost:8000/auth/login`
```json
{
  "email": "ignas.karpusenkovas@gmail.com",
  "password": "t0B7eAEPQEjY00w4MFNUQ1mPk1"
}
```

The above `login` endpoint should return you a `token` which you can then use when getting the protected list of users.  

When executing `GET http://localhost:8000/users`, do not forget to set an authorization header `Authorization: Bearer 'your token here without single quotes'`.
