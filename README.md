<a name="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/AndreMayerT/vest-technical-test">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Vest Technical Test</h3>

  <p align="center">
    A  project made for the Vest team
    <br />

</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#overview">Overview</a></li>
        <li><a href="#communication-and-database">Communication and Database</a></li>
        <li><a href="#data-source">Data Source</a></li>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#additional-notes">Additional Notes</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

This project is the backend of a trading environment simulation, allowing users to interact with virtual stock markets to buy or sell stocks, track portfolio performance, and view historical stock prices.

### Overview

- **API GraphQL Service**: This service provides three main endpoints allowing users to perform market orders, view historical prices of a stock, and track portfolio performance. It was developed using Rust 1.70.0, tokio, warp, async-graphql, and many others. To see the full list of dependencies take a look at the Cargo.toml file.

- **Event Processing Service**: This service operates independently of the API service, executing buy/sell orders and updating the database with fulfilled orders. All logic that persists data to a database is housed within this service, and it was also developed using Rust 1.70.0 and tokio. To see the full list of dependencies take a look at the Cargo.toml file.

### Communication and Database

The two services communicate via a message queue, with the API service sending the order to the processing service. For the message broker, Kafka was used. For the database, PostgreSQL was used.
<br />
<br />
All services and their dependencies, including the database and message broker, are containerized.

### Data Source

Stock data is obtained from the NASDAQ API, providing reliable and up-to-date information about various stocks available in the market.

### Built With

- Rust 1.70.0
- Tokio
- Warp
- Kafka
- GraphQL
- PostgreSQL
- Docker

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

### Pre-Requisites

Since the whole project is containerized, the only pre-requisite is having Docker Engine and Docker Compose installed locally.

- **Docker Engine** <br />
  Please follow the installation guide for your system at https://docs.docker.com/engine/install/

  <br />

- **Docker Compose** <br />
  After having Docker Engine, follow the instructions for installing the Compose plugin at https://docs.docker.com/compose/install/

### Installation

1. Clone the repo
   ```
   git clone https://github.com/AndreMayerT/vest-technical-test.git
   ```
2. Go to the project folder vest-technical-test/ and run the containers
   ```
   cd vest-technical-test/
   ```
   ```
   docker-compose up
   ```

   (please wait for a few minutes to pull all the necessary images and start all the services)

3. That's it!

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Here is examples of the usage for testing the endpoints of the project. For that, there is a built-in and easy to use environment for GraphQL called "playground" which you can use. To go there, after the project is up and running, visit the link http://localhost:8000/playground

<br/>

### 1. Buy or sell a number of shares of a certain stock via its symbol (e.g. 3 shares of AAPL)

Example:

```
mutation {
  placeOrder(input: {
      symbol: "AAPL",
      quantity: 3,
      orderType: BUY
  })
}
```

where:

- symbol: the symbol of the stock that you want to buy or sell.
- quantity: the quantity of shares that you want to buy or sell.
- orderType: the operation that you want to do, BUY being for buying shares and SELL being for selling shares.

<br/>

### 2. Get a list of the stocks you are holding

Usage:

```
query {
    portfolio {
        symbol
        profitLossPercentage
        shareHeld
        currentValue
        referencePrices {
            lowestPrice
            highestPrice
            averagePrice
        }
    }
}
```

<br />

### 3. Get the historic price of a stock you bought in 1-hour intervals

Example:

```
query {
    historicalPrice(symbol: "AAPL") {
        hour
        price
    }
}
```

where:

- symbol: the symbol of the stock that you want to view the historic price.

<br/>

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Contact

Andre Mayer

Email: andremayer.py@gmail.com

LinkedIn: [https://www.linkedin.com/in/andremayert/][linkedin-url]

Project Link: [https://github.com/AndreMayerT/vest-technical-test](https://github.com/AndreMayerT/vest-technical-test)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->

## Additional Notes

This project is not production ready and is only meant to be run locally, the sole purpose of it is to demonstrate my technical skills. Because of that, some precautions were not taken to simplify the process of running locally (e.g. hard coded credentials)

[linkedin-url]: https://linkedin.com/in/andremayert
