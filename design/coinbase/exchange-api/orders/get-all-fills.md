> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get all fills

> Get a list of fills. A fill is a partial or complete match on a specific order.

Get a list of recent fills of the API key's profile.

## API Key Permissions

This endpoint requires either the "view" or "trade" permission.

## Settlement and Fees

Fees are recorded in two stages. Immediately after the matching engine completes a match, the fill is inserted into our datastore. Once the fill is recorded, a settlement process settles the fill and credit both trading counterparties.

The `fee` field indicates the fees charged for this individual fill.

### Liquidity

The `liquidity` field indicates if the fill was the result of a liquidity provider or liquidity taker. `M` indicates Maker and `T` indicates Taker.

### Pagination

Fills are returned sorted by descending `trade_id` from the largest `trade_id` to the smallest `trade_id`. The `CB-BEFORE` header has this first trade ID so that future requests using the `cb-before` parameter fetch fills with a greater trade ID (newer fills).

See [Pagination](/exchange/rest-api/pagination) for more information.


## OpenAPI

````yaml GET /fills
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
  /fills:
    get:
      tags:
        - Orders
      summary: Get all fills
      description: >-
        Get a list of fills. A fill is a partial or complete match on a specific
        order.
      operationId: ExchangeRESTAPI_GetFills
      parameters:
        - name: order_id
          in: query
          description: >-
            limit to fills on a specific order. Either `order_id` or
            `product_id` is required.
          schema:
            type: string
        - name: product_id
          in: query
          description: >-
            limit to fills on a specific product. Either `order_id` or
            `product_id` is required.
          schema:
            type: string
        - name: limit
          in: query
          description: Limit on number of results to return.
          schema:
            type: integer
            format: int64
            default: 100
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
        - name: market_type
          in: query
          description: Market type which the order was filled in.
          schema:
            type: string
            default: spot
            enum:
              - spot
              - rfq
        - name: start_date
          in: query
          description: >-
            Search by minimum posted date time and is inclusive of time
            provided. Valid formats are either RFC3339, date or date time and
            must be after Unix Epoch time.
          schema:
            type: string
        - name: end_date
          in: query
          description: >-
            Search by maximum posted date time and is inclusive of time
            provided. Valid formats are either RFC3339, date or date time and
            must be after Unix Epoch time.
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
                  $ref: '#/components/schemas/apiOrderFill'
        '401':
          description: Unauthorized.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiUnauthorizedResponse'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiOrderFill:
      required:
        - created_at
        - fee
        - funding_currency
        - liquidity
        - order_id
        - price
        - product_id
        - profile_id
        - settled
        - side
        - size
        - trade_id
        - usd_volume
        - user_id
      type: object
      properties:
        trade_id:
          type: integer
          description: id of trade that created the fill
          format: int32
        product_id:
          type: string
          description: book the order was placed on
        order_id:
          type: string
          description: uuid
        user_id:
          type: string
          description: id of user's account
        profile_id:
          type: string
          description: profile_id that placed the order
        liquidity:
          $ref: '#/components/schemas/apiOrderFillLiquidity'
        price:
          type: string
          description: price per unit of base currency
        size:
          type: string
          description: amount of base currency to buy/sell
        fee:
          type: string
          description: fees paid on current filled amount
        created_at:
          type: string
          description: timestamp of fill
          format: date-time
        side:
          $ref: '#/components/schemas/apiOrderSide'
        settled:
          type: boolean
          description: true if funds have been exchanged and settled
        usd_volume:
          type: string
          description: true if funds have been exchanged and settled
        market_type:
          type: string
          description: market type which the order was filled in
        funding_currency:
          type: string
          description: funding currency which the order was filled in
      example:
        created_at: '2019-11-21T01:38:23.878Z'
        trade_id: 78098253
        product_id: BTC-USD
        order_id: 41473628-db2c-464e-b9f4-82df7e4fb4f4
        user_id: 5cf6e115aaf44503db300f1e
        profile_id: 8058d771-2d88-4f0f-ab6e-299c153d4308
        liquidity: T
        price: '8087.38000000'
        size: '0.00601800'
        fee: '0.2433492642000000'
        side: sell
        settled: true
        usd_volume: '48.6698528400000000'
        funding_currency: USDC
    apiUnauthorizedResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
      description: >-
        UnauthorizedResponse is the response message for endpoints in
        rest-gateway that requires authentication.

        This message is used to generate the Exchange REST API documentation
        using OpenAPI format.
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiOrderFillLiquidity:
      type: string
      default: M
      enum:
        - M
        - T
        - O
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