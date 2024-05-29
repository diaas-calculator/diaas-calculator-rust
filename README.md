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

In case something inspires you, here's the top of my todo list (please contact on on slack before starting anything significant): 

- Front + Back: 

  - unit test, ci-cd

  - suggested recipes

  - giving suggestion based on current mix (a specific search request with the current mix diaaas, so that the backend sorts the food items by better match first.)

  - g co2/g of protein

  - equivalent g of complete protein

  - hidden flag: for diaas that we keep for comparison but we want to hide to most users because we believe there is a better result

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

- Reviewing the current dataset, adding more data (see `fixtures/food.csv`)

Things to be careful with: 

- Preparation of the food: see https://www.diaas-calculator.com/diaas-information#limitations-and-precautions
- Type of DIAAS : "real" DIAAS or in vivo or on rat model etc...
- Type of digestive model. From least to most accurate : Apparent ileal digestibility (AID), Standardized ileal digestibility (SID), True ileal digestibility (TID)
- The amino acids are often given in the same order, but not always.
- Age reference pattern: the reference pattern used is often `6month->3years` (it is the default one as per the FAO recommendations). We chose to display the information for the reference pattern `>3y` by default as you can hardly have a 6 month old eat grains and legumes, and mixing grains and legumes are one of the best strategies for what we want to achieve. It is a simple multiplicative factor to get from the 6m->3y to the >3y pattern: 

| his  | iso        | leu        | lys    | met+cys    | phe+tyr    | thr  | trp        | val   |
| ---- | ---------- | ---------- | ------ | ---------- | ---------- | ---- | ---------- | ----- |
| 1,25 | 1,06666667 | 1,08196721 | 1,1875 | 1,17391304 | 1,26829268 | 1,24 | 1,28787879 | 1,075 |

To convert from the *PDCAAS* *2-5 years old* reference pattern to the *DIAAS* *>3years old* reference pattern, use those coefficients: 

| his    | iso        | leu        | lys        | met+cys    | phe+tyr    | thr  | trp        | val   |
| ------ | ---------- | ---------- | ---------- | ---------- | ---------- | ---- | ---------- | ----- |
| 1,1875 | 0,93333333 | 1,08196721 | 1,20833333 | 1,08695652 | 1,53658537 | 1,36 | 1,66666667 | 0,875 |

## Content reviewing

Don't hesitate to reach us should you find any error, be it English error, or scientific fact error.

# Contact us

Don't hesitate to [contact us on slack](https://join.slack.com/t/nouvelespaced-u4p4016/shared_invite/zt-2jknsft6u-4RSF2n09gGkJtXqBvvpQZg) if you have any questions or want to discuss anything about this project.
