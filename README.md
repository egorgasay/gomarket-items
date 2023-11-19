# ItemsAPI Service API  ![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324) ![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white) ![Swagger](https://img.shields.io/badge/Swagger-85EA2D?style=for-the-badge&logo=Swagger&logoColor=black) ![Docker](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

## Overview
ItemsAPI Service API is a robust and easy-to-use API designed to interact with items in market database.

## Features
- Retrieve all items
- Detailed item information including id, name, description, price, and sizes
- Pagination and sorting support
- Validation and error handling

## API Documentation
For detailed API documentation, please refer to the [link](https://egorgasay.github.io/gomarket-items/). It includes comprehensive information about the paths, responses, schemas, and security schemes used in this API.

## Setup and Installation
```shell
git clone https://github.com/egorgasay/gomarket-items
cd gomarket-items
docker-compose up -d
```

## Example
```json
GET http://IP:PORT/v1/items HTTP/1.1
Content-Type: application/json

{
    "offset": 0,
    "limit": 1000,
    "query": {
        "ids": [ 1 ],
        "price": {
            "from": 100.00,
            "to": 2000.000
        },
        "names": {
            "full": [ "Red T-Shirt" ],
            "partly": [ "Red" ]
        }
    },
    "sort_by": {
        "field": "price",
        "desc": false
    }
}

Response
Content-Type: application/json

[
    {
        "id": 1,
        "name": "Red T-Shirt",
        "description": "Cool description",
        "price": 199,
        "sizes": [
            {
                "size": "M",
                "quantity": 5
            }
        ]
    }
]
```

## Contributing
We welcome contributions from the community.

## License
This project is licensed under the *MIT* license.