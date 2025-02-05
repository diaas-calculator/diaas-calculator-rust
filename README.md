# Deprecation warning

This architecture of the DIAAS calculator is no longer maintained

I have swtiched to a 1-tier architecture for now as I don't plan on developing features that require a database and a dedicated backend in the near future. 

Advantages of the 1-tier architecture

- hosting cost is three-fold less
- less hosting resources is more environmental friendly
- app is faster to respond



This rust project works with [DIAAS calculator angular](https://github.com/diaas-calculator/diaas-calculator-angular) using the tag *v1.0.0_3-tier-architecture*

The 1-tier architecture is handled by the same project [DIAAS calculator angular](https://github.com/diaas-calculator/diaas-calculator-angular), *main* branch

# What is this project

This is the rust backend for the [Protein DIAAS calculator](https://www.diaas-calculator.com/)

See also the frontend project [DIAAS calculator angular](https://github.com/diaas-calculator/diaas-calculator-angular)

#  Architecture

- Language: [rust](https://rust-lang.org)
- Dependencies: [cargo](https://doc.rust-lang.org/cargo/guide/dependencies.html)
- Database: [postgresql](https://www.postgresql.org) (dockerized for development)
- Webservice framework: [actix-web](https://actix.rs/) , [actix-web on github](https://github.com/actix/actix-web)
  - The service exposed is a `REST API`
- ORM: [diesel](https://diesel.rs/guides/getting-started.html), [diesel on github](https://github.com/diesel-rs/diesel)

# Running locally

## Database

Use the provided `docker-compose.yml` file to run a local postgresql database: 

```
# first time only
docker network create diaas-db

cd postgres-and-adminer
docker-compose up -d
```

You can browse the database content with adminer at http://localhost:8080 . The credentials and other information can be found in the `docker-compose.yml`. Note that `server` is `db` (the service name)

## Rust backend

- Install rust and cargo: https://rustup.rs/
- Install diesel cli ([ref](https://diesel.rs/guides/getting-started.html)). Example for ubuntu/WSL:

```
sudo apt install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```

- Run the diesel migration to populate the database: 

```
cp .env.localhost .env
diesel migration run
```

- Run the application (configured by `.env` file too)

```
cargo run
```

## Importing the reference dataset

```
cd fixtures
docker run -v $(pwd):/tmp --network diaas-db --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv.load.localhost
docker run -v $(pwd):/tmp --network diaas-db --rm --name pgloader dimitri/pgloader:latest pgloader /tmp/csv.load.i18n.localhost
```

# Accessing the application

API requests can be done against `http://localhost:9090/api` (or `https://diaas-calculator.com/api` in production)

For testing you can use the postman collection provided under `/test`. 

Note that all the endpoints that modify data are deactivated for now (waiting for access control implementation). You can re-activate the missing endpoints in main.rs: 

```
            // waiting for authentication implementation
            .service(food::create)
            .service(food::delete)
            .service(food::update)
```
