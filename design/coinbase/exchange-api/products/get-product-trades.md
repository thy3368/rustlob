> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get product trades

> Gets a list the latest trades for a product.

## Side

The `side` of a trade indicates the maker order side. The maker order is the order that was open on the order book.

A `buy` side indicates a down-tick because the maker was a buy order and their order was removed. A `sell` side indicates an up-tick.

## Pagination

This request is paginated. See [Pagination](/exchange/rest-api/pagination) for more information.


## OpenAPI

````yaml GET /products/{product_id}/trades
openapi: 3.0.1
info:
  title: REST API
  description: >-
    # Welcome to Coinbase Exchange API

    ## Introduction

    The Exchange Trading APIs allow institutions to place orders and access
    account information. The following API pages detail various REST API
    endpoints we offer for lower-frequency trading and general requests.

    ## Getting Started

    To get started, please visit one of the following pages:

    - [Authentication](/exchange/docs/rest-auth)

    - [Rate Limits](/exchange/docs/rest-rate-limits)

    - [Pagination](/exchange/docs/rest-pagination)

    - [Status Codes](/exchange/docs/rest-requests)

    - [Quickstart](/exchange/docs/getting-started)

    ## FIX API

    - [FIX API reference](/exchange/docs/fix-connectivity)

    ## WebSocket API

    - [WebSocket API reference](/exchange/docs/websocket-overview)
  version: '1.0'
servers:
  - url: https://api.exchange.coinbase.com/
security:
  - ApiKeyAuthKey: []
    ApiKeyAuthPassphrase: []
    ApiKeyAuthSign: []
    ApiKeyAuthTimestamp: []
paths:
  /products/{product_id}/trades:
    get:
      tags:
        - Products
      summary: Get product trades
      description: Gets a list the latest trades for a product.
      operationId: ExchangeRESTAPI_GetProductTrades
      parameters:
        - name: product_id
          in: path
          description: list trades for specific product.
          required: true
          schema:
            type: string
        - name: limit
          in: query
          description: Limit on number of results to return.
          schema:
            type: integer
            format: int64
            default: 1000
        - name: before
          in: query
          description: Used for pagination. Sets start cursor to `before` id.
          schema:
            type: string
        - name: after
          in: query
          description: Used for pagination. Sets end cursor to `after` id.
          schema:
            type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/apiProductTrade'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
    apiProductTrade:
      required:
        - price
        - side
        - size
        - time
        - trade_id
      type: object
      properties:
        trade_id:
          type: integer
          format: int32
        side:
          $ref: '#/components/schemas/apiOrderSide'
        size:
          type: string
        price:
          type: string
        time:
          type: string
          format: date-time
      example:
        time: '2020-03-20T00:36:59.860Z'
        trade_id: 86327327
        price: '6225.32000000'
        size: '0.06469797'
        side: sell
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiOrderSide:
      type: string
      default: buy
      enum:
        - buy
        - sell
  securitySchemes:
    ApiKeyAuthKey:
      type: apiKey
      name: cb-access-key
      in: header
    ApiKeyAuthPassphrase:
      type: apiKey
      name: cb-access-passphrase
      in: header
    ApiKeyAuthSign:
      type: apiKey
      name: cb-access-sign
      in: header
    ApiKeyAuthTimestamp:
      type: apiKey
      name: cb-access-timestamp
      in: header

````