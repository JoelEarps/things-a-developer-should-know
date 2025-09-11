# Indexing

The purpose of this document is to show how indexing

## Start up PostgreSQL

```zsh docker run -d \
  --name postgres-container \
  -e POSTGRES_USER=myuser \
  -e POSTGRES_PASSWORD=mypassword \
  -e POSTGRES_DB=mydatabase \
  -p 5432:5432 \
  postgres:16`
```

### Bootstrap the postgres database

The benefits of indexing becomes apparent when you have a large amount of data. The script `.things-a-developer-should-know/data-engineering/bootstrap-sql-analyse.sh` will do the following:

1. Delete the pre existing table if it already exists
2. Create a table called big data.
3. Insert some random data into the table with structures that has some patterns in that will be indexable.

Providing you have `psgl` installed running the following will perform this for you:

`psql -h localhost -U myuser -d mydatabase -a -f ./data-engineering/bootstrap-sql-analyse.sql`

Now if you access the postgres container you should see:

```sql

mydatabase-# \dt
         List of relations
 Schema |   Name   | Type  | Owner  
--------+----------+-------+--------
 public | big_data | table | myuser
(1 row)

mydatabase=# SELECT COUNT(*) FROM big_data;

  count  
---------
 1000000
(1 row)
```

## Run an explain analyse query pre-caching

`EXPLAIN ANALYZE` is a command that will show you two bits of information:

1. `EXPLAIN` - the execution plan for the query.
2. `ANALYZE` - this will execute the plan and show you stats for the query e.g. execution time, time in each table etc.

By using these statements we can see the increase in performance. So lets run an explain analyse query on the newly injected data for trying to find ages of individuals between 30 and 35:

`SELECT * FROM big_data WHERE age BETWEEN 30 AND 35;`

```sql

mydatabase=# EXPLAIN ANALYZE
SELECT * 
FROM big_data
WHERE age BETWEEN 30 AND 35;
                                                    QUERY PLAN                                                     
-------------------------------------------------------------------------------------------------------------------
 Seq Scan on big_data  (cost=0.00..35017.00 rows=101300 width=121) (actual time=0.637..108.207 rows=99403 loops=1)
   Filter: ((age >= 30) AND (age <= 35))
   Rows Removed by Filter: 900597
 Planning Time: 3.696 ms
 Execution Time: 110.805 ms
(5 rows)

```

You want to avoid full table scans. And the more complex queries you do e.g. `LIKE` that does comparison operators

Now lets look at indexing

create index on age that will create a binary tree like query

```sql
CREATE INDEX idx_big_data_age ON big_data(age);
```

```sql
mydatabase=# EXPLAIN ANALYZE                                                           
SELECT * 
FROM big_data
WHERE age = 40;
                                                            QUERY PLAN                                                             
-----------------------------------------------------------------------------------------------------------------------------------
 Bitmap Heap Scan on big_data  (cost=176.29..20008.98 rows=15467 width=121) (actual time=3.734..49.600 rows=16675 loops=1)
   Recheck Cond: (age = 40)
   Heap Blocks: exact=11420
   ->  Bitmap Index Scan on idx_big_data_age  (cost=0.00..172.43 rows=15467 width=0) (actual time=1.691..1.691 rows=16675 loops=1)
         Index Cond: (age = 40)
 Planning Time: 2.533 ms
 Execution Time: 50.306 ms
(7 rows)
```

The time was halved here! and this wasn't even  for a range - this was for everything.

## DB Indexing system design

Data is arrange in database as pages - KBs of data.
When you want to find data in a DB - you pull it into mem and then search each page until you find what you are looking for - which can be slow for larger data!
There are other optimisation techniques.

Indexes are data structures that act as a map (stored on disk) to direct us to the item we are looking for mroe quickly i.e. point us to the page.

BTree index - basic trees, where each parent node points to a different page. So the best example is the range based finding with ages.
Hash Index - Hash map, that exists in disk. Pass the email to hash function, where the hash is a pointer to where the data exists on disk. Rarely actually used in index's. Btrees are preferred as the offer almost the same performance for matching queries whilst also supporting range queries.
Geospatial - wouldn't use a btree here e.g. search for latitude or longitude. The do not excel at 2D. Well how would approach:

1. Goe-hashing - basically turn things into 1D string, so UK for example, lets call that 1, Manchester could be 11, London 12, then you keep adding precision. Create a hash then a btree on top of them. Typically they will be base32.
2. Quad tress - split recursively again. This is then mapped to a tree, and we go deeper into tree as they place we want to search becomes more densely populated.
3. R tress - derived from Qaud trees - does clustering that are close and then you group them that way. Postgis uses this.

Btrees are not good for sub string searches. Here you use an inverted index - they are great for sub string searching. SO lets sy you want to find a string called %Joel% - you will link the sub string %Joel% to every page that contains this sub string.
