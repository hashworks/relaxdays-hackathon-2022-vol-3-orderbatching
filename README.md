# orderbatching

Solution for a warehouse-problem of the RelaxDays summer hackathon in 2022.

## How to run this project

You can get a running version of this code by using:

```bash
docker build -t orderbatching-kromlinger-justin .
docker run -v $(PWD)/instances:/instances:ro -v $(PWD)/solutions:/solutions:rw orderbatching-kromlinger-justin /instances/instance_1.json /solutions/solution_1.json
```
