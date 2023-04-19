.PHONY: build build-ims build-passenger-service build-flight-service run run-ims run-passenger-service run-flight-service

# Build all services
build: build-passenger-service build-flight-service

# Build passenger_service service
build-passenger-service:
	cd api/passenger_service && cargo build

# Build flight_service service
build-flight-service:
	cd api/immigration_service && cargo build

# Run all services
run: run-passenger-service run-flight-service

# Run passenger_service service
run-passenger-service:
	cd api/passenger_service && cargo run

# Run flight_service service
run-flight-service:
	cd api/immigration_service && cargo run

# Restart DB
db-restart:
	docker stop roach1 && docker rm roach1 
	docker stop roach2 && docker rm roach2 
	docker stop roach3 && docker rm roach3 

	docker run -d --name=roach1 --hostname=roach1 --net=roachnet -p 26257:26257 -p 8080:8080  -v "roach1:/cockroach/cockroach-data"  cockroachdb/cockroach:v22.2.7 start --insecure --join=roach1,roach2,roach3
	docker run -d --name=roach2 --hostname=roach2 --net=roachnet -v "roach2:/cockroach/cockroach-data" cockroachdb/cockroach:v22.2.7 start --insecure --join=roach1,roach2,roach3
	docker run -d --name=roach3 --hostname=roach3 --net=roachnet -v "roach3:/cockroach/cockroach-data" cockroachdb/cockroach:v22.2.7 start --insecure --join=roach1,roach2,roach3