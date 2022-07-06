# orderbatching

Winning solution for a warehouse-problem of the RelaxDays summer hackathon in 2022.

## How to run this project

You can get a running version of this code by using:

```bash
docker build -t orderbatching-kromlinger-justin .
docker run -v $(pwd)/instances:/instances:ro -v $(pwd)/solutions:/solutions:rw orderbatching-kromlinger-justin /instances/instance1.json /solutions/solution1.json
```

Due to randomized runs and a hard 5 minutes runtime limit provided by RelaxDays it will always run for about 4 minutes.

## Findings

### Cost-function

Expenses are in that order:
* Amount of visitied warehouses → group orders by warehouses
* Amount of waves → fill up waves to capacity
* Amount of aisles → group orders by warehouse-aisles
* Amount of batches -> fill up batches to capacity

### Order sizes are equally distributed

Due to the generator algorithm RelaxDays is using the order size always goes from 1 to 9 and is equally distributed:
```
$ jq '.Orders[].ArticleIds | length' instances/instance4.json | sort -r | uniq -c
   7259 9
   7206 8
   7300 7
   7188 6
   7364 5
   7313 4
   7250 3
   7385 2
   7271 1
```

This enables us to merge 9s with 1s, 8s with 2s, etc. – however, this might not be the case for other instances.

### Distribution of Order-Volumes

https://fb.kromlinger.eu/WrIOxZy/

Not sure what to make of it yet.

### Distribution of Order-Warehouses

Makes sense: In the worst case, all items are in a different warehouse.

```
    417 9
   2876 8
   7044 7
   9996 6
  10325 5
   9700 4
   9015 3
   8426 2
   7737 1
```

### Distribution of Articles in Warehouses

Equally distributed, the number of warehouses is quite small.

```
   4096 15
   4096 14
   4096 13
   4096 12
   4096 11
   4096 10
   4096 9
   4096 8
   4096 7
   4096 6
   4096 5
   4096 4
   4096 3
   4096 2
   4096 1
   4096 0
```