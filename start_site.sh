#!/bin/bash

# Start the backend
cd backend
export GIN_MODE=release
go build
./batt_log &

cd ../frontend
npm start &
