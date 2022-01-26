# Capbreton Parking App

## Run server
```bash
cargo run --bin capark-server
```
Then go to ```http://localhost:8000/```

## Run app
```bash
trunk serve ./caparking_app/index.html --proxy-backend http://localhost:8000/api/
```
Then go to ```http://localhost:8080/```


## Setup dev env
Create a simlink from the dist folder of the webapp to the web-app folder of the server
```bash
cd caparking_server/resources/
ln -s ../../caparking_app/dist/ web-app
```