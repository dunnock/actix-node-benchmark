
## Summary

Presume we have working node.js server with postgresql and redis DB, table tasks with assignee. Presume we follow CQRS principle and looking to optimize all the GET queries using fastest web framework [actix-web](). Let's analyse potential performance gains and hosting capacity savings:
1. How much can we gain on GET object queries
2. How much can we gain performing GET search queries
3. How much can we gain employing data locality and searches


When working with cache we will consider that 1% of keys stored are randomly invalidated, presuming that cache invalidation is a parallel process outside of scope of this excercise.

For the data locality case we will presume that there is existing feed of state change events, which would allow to reconstruct data in the local search index and cache. We will consider random update for every 100 requests.

For ease of setup everything will be hosted in docker via docker-compose.