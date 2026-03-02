> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get product ticker

> Gets snapshot information about the last trade (tick), best bid/ask and 24h volume.

## Real-time updates

Coinbase recommends that you get real-time updates by connecting with the WebSocket stream and listening for match messages, rather than polling.


## OpenAPI

````yaml GET /products/{product_id}/ticker
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
  /products/{product_id}/ticker:
    get:
      tags:
        - Products
      summary: Get product ticker
      description: >-
        Gets snapshot information about the last trade (tick), best bid/ask and
        24h volume.
      operationId: ExchangeRESTAPI_GetProductTicker
      parameters:
        - name: product_id
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiProductTicker'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
    apiProductTicker:
      required:
        - ask
        - bid
        - price
        - size
        - time
        - trade_id
        - volume
      type: object
      properties:
        ask:
          type: string
        bid:
          type: string
        volume:
          type: string
        trade_id:
          type: integer
          format: int32
        price:
          type: string
        size:
          type: string
        time:
          type: string
          format: date-time
        rfq_volume:
          type: string
        conversions_volume:
          type: string
      example:
        trade_id: 86326522
        price: '6268.48'
        size: '0.00698254'
        time: '2020-03-20T00:22:57.833Z'
        bid: '6265.15'
        ask: '6267.71'
        volume: '53602.03940154'
        rfq_volume: '123.122'
        conversions_volume: '0.00'
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
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