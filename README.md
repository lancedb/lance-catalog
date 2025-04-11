# Lance Catalog

**Lance Catalog** is an open specification on top of the storage-based Lance open format
to standardize access to a collection of Lance tables.
It describes how a catalog service like Apache Hive MetaStore (HMS), Apache Gravitino, etc.
should store and use Lance tables, as well as how ML/AI tools and analytics compute engines
should access Lance tables.

## Repository structure

| Directory                                                                      | Description                                         |
|--------------------------------------------------------------------------------|-----------------------------------------------------|
| [spec](./spec)                                                                 | Lance Catalog OpenAPI specification                 |
| [rust/lance-catalog-reqwest-client](./rust/lance-catalog-reqwest-client)       | Generated Rust reqwest client for Lance Catalog     |
| [java/lance-catalog-apache-client](./java/lance-catalog-apache-client)         | Generated Java Apache HTTP client for Lance Catalog |
| [java/lance-catalog-springboot-server](./java/lance-catalog-springboot-server) | Generated Java SpringBoot server for Lance          |

## Development Guide

### Install OpenAPI Generator

We use [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator) 
to generate various clients and servers for the catalog specification.
We recommend installing the tool through [pip](https://pypi.org/project/openapi-generator-cli/) 
for consistent experience across platforms.
First time setup of virtual environment and installation:

```bash
python3 -m venv .env
source .env/bin/activate
pip install -r requirements.txt
```

### Develop Rust

```bash
cd rust

# clean all existing generated modules
make clean

# clean, and then generate all clients and servers
make gen 

# clean and generate the rust reqwest client
make gen-rust-reqwest-client

# clean, generate and build all clients and servers
make build
```

### Develop Java

```bash
cd java

# clean all existing generated modules
make clean

# clean, and then generate all clients and servers
make gen 

# clean and generate the Java Apache client
make gen-java-apache-client

# clean and generate the Java Spring Boot server
make gen-java-springboot-server

# clean, generate and build all clients and servers
make build
```
