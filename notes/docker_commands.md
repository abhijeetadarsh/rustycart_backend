# Docker commands to setup mongodb

- The `mongodb` service runs the official MongoDB image, exposes port 27017, and mounts a volume for data persistence.
- The `mongo-express` service runs the Mongo Express image, exposes port 8081, and specifies environment variables for connecting to the MongoDB instance.
- Both services are connected to the `my_network` network for communication between containers.
- The `volumes` section defines the `mongodb_data` volume for MongoDB data persistence.

To start both containers, navigate to the directory containing the `docker-compose.yml` file and run:

```bash
sudo docker-compose up -d
```

This command will start both the MongoDB and Mongo Express containers in detached mode. Once they're running, you can access Mongo Express at `http://localhost:8081` in your web browser. 

To stop and remove the containers, run:

```bash
sudo docker-compose down
```

This will stop and remove both containers, but it will keep the volume intact, preserving your MongoDB data.