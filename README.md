# What is this project

This is the rust backend for the [Protein DIAAS calculator](https://www.diaas-calculator.com/)

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

- API requests can be done against `http://localhost:9090/api`

For testing you can use the postman collection provided under `/test`

Note that all the endpoints that modify data are deactivated for now (waiting for access control implementation). You can re-activate the missing endpoints in main.rs: 

```
            // waiting for authentication implementation
            .service(food::create)
            .service(food::delete)
            .service(food::update)
```

# Contributing

## Code

This is an opensource project. All the contributions are welcome!

In case something inspires you, here's the top of my todo list (please open an issue to mention you're working on it if you do): 

- Front + Back: 

  - unit test, ci-cd

  - suggested recipes

  - g co2/g of protein

  - equivalent g of complete protein

  - user profile

  - personal recipes (shared or not)

  - personal food items

  - mode compare: food items and/or mix

  - add more details about studies in case links go dead (name, authors, date, journal). new table?

  - special editor profiles who are allowed to update/delete/create food items (with validation of the administrator)


- Front

  - display details about the types of DIAAS. ex DIAAS-rat-> DIAAS on rodents

  - display in the main page the type of measure (diaas, pdcaas...). using a logo?

  - contextual help (especially for mobile devices)

  - search by type of food and other filters

  - add "clear mix" button

  - switch to other age diaas (it is a simple multiplication)

  - search that starts running without entering "search" button (after a pause in typing)

  - more elements translated in the GUI

## Translation

You can help with food item translation into other languages. See for example `fixtures/food_i18n.csv` for the expected format

## Dataset

Gathering the `DIAAS` data is a tedious and error-prone process. Contributions could be :

- Reviewing the current dataset
- Adding more data (see `fixtures/food.csv`)

Things to be careful with: 

- Preparation of the food: see https://www.diaas-calculator.com/diaas-information#limitations-and-precautions
- Type of DIAAS : "real" DIAAS or in vivo or on rat model etc...

It is perfectly acceptable to add an item that is already present in the database, but then the characteristics must be different so that users can choose the appropriate one (ex: cooked vs crude, soaked or not...). If everything is the same, please keep a single entry, the one that seems the most accurate (or compute an average but currently we have only a single link available in the details)



# Contact us

We're currently looking for a slack or similar community to attach to. In the meantime you can always reach us by opening an issue on github.