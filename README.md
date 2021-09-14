# fakeset


## what?

A postgres extension to generate fake data.

## why?

It's always hard to generate fake data in pure sql,  
and I wanted to try https://github.com/zombodb/pgx :)

## how?

build the extension, and then:


```sql
create extension fakeset;
```

### generate 2000 sentences that are between 3 and 10 words

```sql
select lorem(3, 10) from generate_series(1, 2000);
```
