# Dev Frontend

```sh
npm run build -- -w
```


# Dev Backend Test Model All

<!-- ```sh
# Test for the database
`cargo watch -q -c -w backend/passenger_service/src/ -x 'test model_db_ -- --test-threads=1 --nocapture'`
```

```sh
# Test for the passenger
cargo watch -q -c -w backend/passenger_service/src/ -x 'test model_passenger_ -- --test-threads=1 --nocapture'
```  -->

```sh
# test for the model (db + passenger)
cargo watch -q -c -w backend/passenger_service/src/ -x 'test model_ -- --test-threads=1 --nocapture'
```

# Dev Web

```sh
# Test for the web
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
```

## run this in backend/passenger_service
```sh
cargo watch -q -c -w src/ -x 'run -- ../../frontend/web'
```

## CockroachDB docker (insecure - dev only
#### Docker networg bridge
```sh
docker network create -d bridge roachnet
```
#### Docker volumes
```
docker volume create roach1
docker volume create roach2
docker volume create roach3
```
### Run docker

```sh
docker run -d --name=roach1 --hostname=roach1 --net=roachnet -p 26257:26257 -p 8080:8080  -v "roach1:/cockroach/cockroach-data"  cockroachdb/cockroach:v22.2.7 start --insecure --join=roach1,roach2,roach3
```

```sh
docker run -d --name=roach2 --hostname=roach2 --net=roachnet -v "roach2:/cockroach/cockroach-data" cockroachdb/cockroach:v22.2.7 start --insecure --join=roach1,roach2,roach3
```

```sh
docker run -d --name=roach3 --hostname=roach3 --net=roachnet -v "roach3:/cockroach/cockroach-data" cockroachdb/cockroach:v22.2.7 start --insecure --join=roach1,roach2,roach3
```
#### One-time initialization of the cluster
```sh
docker exec -it roach1 ./cockroach init --insecure
```
#### Get Db details
```sh
docker exec -it roach1 grep 'node starting' cockroach-data/logs/cockroach.log -A 11
```
#### Run the SQL shell
```sh
docker exec -it roach1 ./cockroach sql --insecure
```

#### Destroying the cluster
```sh
docker stop roach1 && docker rm roach1 
docker stop roach2 && docker rm roach2 
docker stop roach3 && docker rm roach3 
```

