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

# Contributing

## Code

This is an opensource project. All the contributions are welcome!

In case something inspires you, here's the top of my todo list (please contact me on slack before starting anything significant): 

- Front + Back: 
  - add food as doses by default instead of 100g (1 egg, 1 portion of xxx = x grams)
  - pagination (don't display all food items, just 10 or 20 and paginate the next results)
  - giving suggestion based on current mix (a specific search request with the current mix diaaas, so that the backend sorts the food items by better match first.)
  - equivalent g of complete protein
  - crude to cooked factor + in app conversion
  - unit test, ci-cd
  - user profile
  - personal recipes (shared or not)
  - personal food items
  - mode compare: food items and/or mix
  - add more details about studies in case links go dead (name, authors, date, journal). new table?
  - special editor profiles who are allowed to update/delete/create food items (with validation of the administrator)



- Front

  - new page about carbon footprint
    - the impact of food on the carbon footprint in general
    - specifically the impact of meat and dairy (compared to transport / packaging)
  - android/IOS app
  - display details about the types of DIAAS in the detailed view. ex DIAAS-rat-> DIAAS on rodents
  - switch to other age diaas (it is a simple multiplication)
  - search that starts running without entering "search" button (after a pause in typing)
  - more elements translated in the GUI

## Translation

You can help with food item translation into other languages. See for example `fixtures/food_i18n.csv` for the expected format

## Dataset

You can enrich our current dataset by adding lines in the `fixtures/food.csv` and `fixtures/food_i18n.csv` files and directly send us what you would like to see added.

### Points of attention

- Preparation of the food: see https://www.diaas-calculator.com/diaas-information#limitations-and-precautions
- Score standard, animal model, digestibility model : see the [user manual](http://localhost:4200/user-manual#information-about-food-items)
- Age reference pattern: the reference pattern used is often `6month->3years` (it is the default one as per the FAO recommendations). We display the information for the reference pattern `>3y` by default. It is a simple multiplicative factor to get from the 6m->3y to the >3y pattern: 

| his  | iso        | leu        | lys    | met+cys    | phe+tyr    | thr  | trp        | val   |
| ---- | ---------- | ---------- | ------ | ---------- | ---------- | ---- | ---------- | ----- |
| 1,25 | 1,06666667 | 1,08196721 | 1,1875 | 1,17391304 | 1,26829268 | 1,24 | 1,28787879 | 1,075 |

To convert from the *PDCAAS* *2-5 years old* reference pattern to the *DIAAS* *>3 years old* reference pattern, use those coefficients: 

| his    | iso        | leu        | lys        | met+cys    | phe+tyr    | thr  | trp        | val   |
| ------ | ---------- | ---------- | ---------- | ---------- | ---------- | ---- | ---------- | ----- |
| 1,1875 | 0,93333333 | 1,08196721 | 1,20833333 | 1,08695652 | 1,53658537 | 1,36 | 1,66666667 | 0,875 |

### Computing the DIAAS

Sometimes it is required to gather data from different sources and compute the DIAAS, especially when it comes to processing (extrusion, baking, cooking...) for which data is really scarse. Or it may be required when a study results are incoherent or when some data is missing or not publicly available.

To do this, the excel sheet in this meta-study is really helping: https://data.mendeley.com/datasets/gz3cx7d5f4/1

Feel free to ask my latest version on slack. I have also initiated one for PDCAAS scores.

## Content reviewing

Feel free to reach us should you find any error

# Contact us

Feel free to [contact us on slack](https://join.slack.com/t/nouvelespaced-u4p4016/shared_invite/zt-2jknsft6u-4RSF2n09gGkJtXqBvvpQZg) if you have any questions or want to discuss anything about this project.
