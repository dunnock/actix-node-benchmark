
## Summary

Presume we have working node.js server with postgresql and redis DB, table task with assignee. We and looking to optimize all the GET queries using fastest web framework [actix-web](). Let's analyse potential performance gains and hosting capacity savings:

For ease of setup database will be hosted in docker via docker-compose.

## Start the database

```
docker-compose up database
```

## Fill the database

```
cargo run --bin actix-raw --release
... from another console ...
curl -XPOST "http://127.0.0.1:3001/actions/filldb?workers=100&tasks=100000"
...after it completes server can be shutdown
```

## Run actix server

```
cargo run --bin actix-bench --release
```

## Run node server

```
cd node
npm install
npm run start
```

## Running benchmarks
```
cargo run --bin benchit --release -- --help
```