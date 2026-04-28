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

### step 5
fix devcontainer issue by using `dockerComposeFile` option in [devcontainer.json](./.devcontainer/devcontainer.json)

`dockerComposeFile` uses an array of docker-compose.yaml files

../docker-compose.yaml is first in the list, so the project root becomes the current working director

.env is also picked up correctly from the project root

Note: it takes about 45 seconds to access the pgadmin. Look for Starting gunicorn 23.0.0 and Listening at: http://[::]:5050 (1)

## TODO
fix health check for postgres and pgadmin. Add postgres dependency to pgadmin
fix ports
```json
{
  "name": "My Dev Container",
  "dockerComposeFile": "docker-compose.yml",
  "service": "dev",
  "forwardPorts": [
    3000,
    "db:5432"
  ],
  "portsAttributes": {
    "db:5432": {
      "label": "PostgreSQL Database"
    }
  }
}   
```

