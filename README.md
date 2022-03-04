# rust_mongo_http_PoC
Containerized Rust app that returns all Mongo's databases as a HTTP service


### How to run
```bash
$ COMPOSE_DOCKER_CLI_BUILD=1 DOCKER_BUILDKIT=1 docker-compose up -d --build
```

### Test that is working
```bash
$ curl http://localhost:8080
Databases:
- admin
- config
- local
```

### Details
Rust code is in [main.rc](backend/src/main.rc) file.  
The HTTP service is listening on TCP 8080, on 0.0.0.0.  
Rust is getting the Mongo URI from the following environment variable, placed in [.env](backend/.env)
`MONGODB_URI`

The containers are deployed using docker-compose with BuildKit, using multi-stage builds for Rust. See [Dockerfile](backend/Dockerfile)

### Troubeshooting
Check that the containers are up & running:
```bash
$ docker-compose ps
```
Testing the mongo connection after running the `docker-compose up` command from "How to run";
```bash
$ mongo 127.0.0.1:27017
```
Connect to one of the containers:
```shell
$ docker exec -ti mongo /bin/bash
```

### Cleanup
```bash
$ docker-compose stop
$ docker-compose rm
```


