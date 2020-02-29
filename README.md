
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

Below is output on Ubuntu 18 running on 40CPU Xeon

```
concurrency = 1
node ,  1,      0.33,   237,    0.56,    87,    0.00,     7,    1.91,   524
actix,  1,      0.42,   237,    0.00,    89,    0.29,     7,    0.99,   1003
concurrency = 2
node ,  2,      0.57,   237,    0.91,    87,    0.00,     7,    2.12,   949
actix,  2,      0.82,   237,    0.00,    88,    0.41,     7,    1.04,   1915
concurrency = 4
node ,  4,      0.70,   266,    1.02,    87,    0.00,     7,    3.41,   1181
actix,  4,      1.98,   237,    0.00,    86,    0.78,     7,    0.89,   4466
concurrency = 8
node ,  8,      0.73,   325,    1.02,    90,    0.00,     7,    6.69,   1199
actix,  8,      3.34,   237,    0.01,    47,    0.99,     7,    1.10,   7233
concurrency = 16
node ,  16,     0.73,   355,    1.02,    88,    0.00,     7,    12.94,  1237
actix,  16,     3.58,   237,    0.00,    88,    1.00,     7,    2.05,   7783
concurrency = 32
node ,  32,     0.77,   355,    1.02,    85,    0.00,     7,    24.25,  1319
actix,  32,     3.53,   237,    0.00,    85,    1.00,     8,    4.21,   7587
concurrency = 64
node ,  64,     0.78,   354,    1.03,    87,    0.00,     8,    45.49,  1383
actix,  64,     3.50,   237,    0.00,    46,    1.00,     9,    8.42,   7475
concurrency = 128
node ,  128,    0.80,   355,    1.03,    89,    0.00,     9,    85.85,  1361
actix,  128,    3.45,   237,    0.00,    48,    1.00,    12,    15.83,  7387
concurrency = 256
node ,  256,    0.77,   354,    1.05,    89,    0.00,    12,    182.21, 1282
actix,  256,    3.31,   237,    0.00,    46,    1.00,    18,    31.95,  7320
```