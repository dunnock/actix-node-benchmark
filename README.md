
## Summary

Presume we have working node.js server with postgresql and redis DB, table task with assignee. We and looking to optimize all the GET queries using fastest web framework [actix-web](). Let's analyse potential performance gains and hosting capacity savings:

For ease of setup database will be hosted in docker via docker-compose.

# Project structure

- node-bench - simple implementation without optimizations built with Express and Knex based on [provided example](https://github.com/robmclarty/knex-express-project-sample)
- actix-bench - simple implementation without optimizations built with Actix-web and Deadpool based on [async-pg example](https://github.com/actix/examples/tree/master/async_pg)
- actix-raw - low level implementation based on Actix-web and [Tokio-postgres]()
- node-raw - low level implementation based on Express and [Node-postgres](https://node-postgres.com/)

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
cd node-bench
npm install
npm run start
```

## Running benchmarks
```
cargo run --bin benchit --release -- --help
```

Below is output on Ubuntu 18 running on 40CPU Xeon

```
Target, Concur, PG cpu, mem,    ND cpu, mem,    AX cpu, mem,    lat ms, rps
Starting test /tasks
concurrent load = 1
node ,  1,      0.33,    83,    0.56,    81,    0.00,     4,    2.06,   496
actix,  1,      0.47,    97,    0.00,    83,    0.29,     5,    0.96,   1037
concurrent load = 2
node ,  2,      0.58,   112,    0.91,    86,    0.00,     5,    2.19,   919
actix,  2,      0.89,   126,    0.00,    86,    0.42,     5,    0.98,   2022
concurrent load = 4
node ,  4,      0.68,   155,    1.02,    87,    0.00,     5,    3.58,   1123
actix,  4,      2.01,   183,    0.00,    87,    0.80,     5,    0.88,   4520
concurrent load = 8
node ,  8,      0.76,   242,    1.02,    87,    0.00,     5,    6.51,   1232
actix,  8,      3.42,   298,    0.00,    87,    0.99,     6,    1.06,   7525
concurrent load = 16
node ,  16,     0.73,   395,    1.02,    87,    0.00,     6,    13.10,  1221
actix,  16,     3.89,   469,    0.00,    87,    1.00,     6,    1.98,   8076
concurrent load = 32
node ,  32,     0.75,   498,    1.02,    86,    0.00,     6,    23.83,  1342
actix,  32,     3.61,   498,    0.00,    87,    1.00,     7,    4.01,   7963
concurrent load = 64
node ,  64,     0.82,   498,    1.03,    86,    0.00,     7,    45.26,  1389
actix,  64,     3.54,   498,    0.00,    87,    1.00,     9,    7.97,   7894
concurrent load = 128
node ,  128,    0.82,   498,    1.03,    88,    0.00,     9,    83.69,  1393
actix,  128,    3.48,   498,    0.00,    89,    1.00,    12,    14.99,  7793
concurrent load = 256
node ,  256,    0.78,   498,    1.05,    91,    0.00,    12,    180.55, 1291
actix,  256,    3.56,   498,    0.00,    91,    1.00,    18,    30.18,  7742
concurrent load = 512
node ,  512,    0.76,   498,    1.10,   104,    0.00,    18,    369.69, 1246
actix,  512,    3.20,   498,    0.02,    48,    1.00,    26,    68.07,  6862

Latency in ms (lower is better)

concurrent load 1
node   |*                                                                                                     |
actix  |*                                                                                                     |

concurrent load 2
node   |*                                                                                                     |
actix  |*                                                                                                     |

concurrent load 4
node   |*                                                                                                     |
actix  |*                                                                                                     |

concurrent load 8
node   |**                                                                                                    |
actix  |*                                                                                                     |

concurrent load 16
node   |****                                                                                                  |
actix  |*                                                                                                     |

concurrent load 32
node   |*******                                                                                               |
actix  |**                                                                                                    |

concurrent load 64
node   |*************                                                                                         |
actix  |***                                                                                                   |

concurrent load 128
node   |***********************                                                                               |
actix  |*****                                                                                                 |

concurrent load 256
node   |*************************************************                                                     |
actix  |*********                                                                                             |

concurrent load 512
node   |***************************************************************************************************** |
actix  |*******************                                                                                   |

Requests per second (higher is better)

concurrent load 1
node   |*******                                                                                               |
actix  |*************                                                                                         |

concurrent load 2
node   |************                                                                                          |
actix  |**************************                                                                            |

concurrent load 4
node   |**************                                                                                        |
actix  |********************************************************                                              |

concurrent load 8
node   |****************                                                                                      |
actix  |**********************************************************************************************        |

concurrent load 16
node   |****************                                                                                      |
actix  |***************************************************************************************************** |

concurrent load 32
node   |*****************                                                                                     |
actix  |***************************************************************************************************   |

concurrent load 64
node   |******************                                                                                    |
actix  |**************************************************************************************************    |

concurrent load 128
node   |******************                                                                                    |
actix  |*************************************************************************************************     |

concurrent load 256
node   |****************                                                                                      |
actix  |************************************************************************************************      |

concurrent load 512
node   |****************                                                                                      |
actix  |*************************************************************************************                 |

Starting test /tasks?summary=wherever&full=true&limit=10
concurrent load = 1
node ,  1,      0.94,   456,    0.07,    57,    0.00,    28,    35.84,  27
actix,  1,      0.87,   2048,   0.00,    57,    0.13,    28,    6.84,   170
concurrent load = 2
node ,  2,      1.89,   2153,   0.12,    59,    0.00,    28,    39.89,  50
actix,  2,      1.77,   2153,   0.00,    68,    0.20,    28,    5.77,   346
concurrent load = 4
node ,  4,      3.75,   2363,   0.20,    70,    0.00,    28,    42.20,  94
actix,  4,      3.62,   2363,   0.00,    86,    0.34,    28,    5.92,   674
concurrent load = 8
node ,  8,      7.59,   2829,   0.37,    84,    0.00,    28,    45.49,  175
actix,  8,      7.20,   2828,   0.00,    87,    0.58,    28,    6.12,   1305
concurrent load = 16
node ,  16,     14.63,  3668,   0.59,    93,    0.00,    28,    48.58,  328
actix,  16,     13.50,  3668,   0.00,    88,    0.88,    29,    6.43,   2486
concurrent load = 32
node ,  32,     14.62,  3669,   0.60,    89,    0.00,    29,    97.28,  328
actix,  32,     13.47,  3668,   0.00,    91,    0.89,    31,    13.35,  2395
concurrent load = 64
node ,  64,     14.61,  3668,   0.59,    92,    0.00,    31,    192.18, 326
actix,  64,     13.48,  3668,   0.00,    88,    0.89,    31,    25.91,  2425
concurrent load = 128
node ,  128,    14.29,  3668,   0.64,    91,    0.00,    31,    360.66, 322
actix,  128,    13.52,  3668,   0.00,    91,    0.89,    32,    48.44,  2410
concurrent load = 256
node ,  256,    14.25,  3668,   0.63,   102,    0.00,    32,    714.51, 323
actix,  256,    13.47,  3668,   0.00,    52,    0.90,    34,    95.73,  2437
concurrent load = 512
node ,  512,    14.18,  3668,   0.64,   107,    0.00,    34,    1430.00,316
actix,  512,    13.52,  3668,   0.00,    55,    0.88,    38,    198.89, 2345

Latency in ms (lower is better)

concurrent load 1
node   |***                                                                                                   |
actix  |*                                                                                                     |

concurrent load 2
node   |***                                                                                                   |
actix  |*                                                                                                     |

concurrent load 4
node   |***                                                                                                   |
actix  |*                                                                                                     |

concurrent load 8
node   |****                                                                                                  |
actix  |*                                                                                                     |

concurrent load 16
node   |****                                                                                                  |
actix  |*                                                                                                     |

concurrent load 32
node   |*******                                                                                               |
actix  |*                                                                                                     |

concurrent load 64
node   |**************                                                                                        |
actix  |**                                                                                                    |

concurrent load 128
node   |**************************                                                                            |
actix  |****                                                                                                  |

concurrent load 256
node   |**************************************************                                                    |
actix  |*******                                                                                               |

concurrent load 512
node   |***************************************************************************************************** |
actix  |**************                                                                                        |

Requests per second (higher is better)

concurrent load 1
node   |**                                                                                                    |
actix  |*******                                                                                               |

concurrent load 2
node   |***                                                                                                   |
actix  |**************                                                                                        |

concurrent load 4
node   |****                                                                                                  |
actix  |****************************                                                                          |

concurrent load 8
node   |********                                                                                              |
actix  |*****************************************************                                                 |

concurrent load 16
node   |**************                                                                                        |
actix  |***************************************************************************************************** |

concurrent load 32
node   |**************                                                                                        |
actix  |*************************************************************************************************     |

concurrent load 64
node   |**************                                                                                        |
actix  |**************************************************************************************************    |

concurrent load 128
node   |*************                                                                                         |
actix  |*************************************************************************************************     |

concurrent load 256
node   |*************                                                                                         |
actix  |***************************************************************************************************   |

concurrent load 512
node   |*************                                                                                         |
actix  |***********************************************************************************************       |
```