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