> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get single product

> Get information on a single product.



## OpenAPI

````yaml GET /products/{product_id}
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
  /products/{product_id}:
    get:
      tags:
        - Products
      summary: Get single product
      description: Get information on a single product.
      operationId: ExchangeRESTAPI_GetProduct
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
                $ref: '#/components/schemas/apiProduct'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
    apiProduct:
      required:
        - auction_mode
        - base_currency
        - base_increment
        - cancel_only
        - display_name
        - id
        - limit_only
        - margin_enabled
        - min_market_funds
        - post_only
        - quote_currency
        - quote_increment
        - status
        - status_message
      type: object
      properties:
        id:
          type: string
        base_currency:
          type: string
        quote_currency:
          type: string
        quote_increment:
          type: string
          description: Min order price (a.k.a. price increment
        base_increment:
          type: string
        display_name:
          type: string
        min_market_funds:
          type: string
        margin_enabled:
          type: boolean
        post_only:
          type: boolean
        limit_only:
          type: boolean
        cancel_only:
          type: boolean
        status:
          $ref: '#/components/schemas/apiProductStatusEnum'
        status_message:
          type: string
        trading_disabled:
          type: boolean
        fx_stablecoin:
          type: boolean
        max_slippage_percentage:
          type: string
        auction_mode:
          type: boolean
        high_bid_limit_percentage:
          type: string
          description: >-
            Percentage to calculate highest price for limit buy order (Stable
            coin trading pair only)
      example:
        id: BTC-USD
        base_currency: BTC
        quote_currency: USD
        quote_increment: '0.01000000'
        base_increment: '0.00000001'
        display_name: BTC/USD
        min_market_funds: '10'
        margin_enabled: false
        post_only: false
        limit_only: false
        cancel_only: false
        status: online
        status_message: ''
        auction_mode: true
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiProductStatusEnum:
      type: string
      default: online
      enum:
        - online
        - offline
        - internal
        - delisted
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