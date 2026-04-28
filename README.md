# codespace-samples

## steps

### step 1

Create devcontainer config using rust bookworm image
 
```shell
cargo init
cargo run
```

### step 2

```shell
# add postgres dependency
cargo add postgres

# update main.rs to connect to postgres and run a select

# Note: if you change the username, password or dbname then reset the docker volume using docker compose down -v
# start the postgres db.
docker compose up -d

# run the rust app, should return Result: 2
cargo run
```

### step 3

add pgadmin service to docker-compose.yaml
rename the postgres service to db to stay consistence with other projects

### step 4
use .env environment variables for docker-compose and rust app
```shell
cargo add dotenvy
```