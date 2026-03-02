> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get all known trading pairs

> Gets a list of available currency pairs for trading.

<Warning>
  **Order Size Limits Removed**

  The properties `base_max_size`, `base_min_size`, `max_market_funds` were [removed on June 30](/exchange/changes/changelog#2022-jun-30).

  The property, `min_market_funds`, has been repurposed as the notional minimum size for limit orders.
</Warning>

The `base_min_size` and `base_max_size` fields define the min and max order size.

The `min_market_funds` and `max_market_funds` fields define the min and max funds allowed in a market order.

`status_message` provides any extra information regarding the status if available.

The `quote_increment` field specifies the min order price as well as the price increment.

The order price must be a multiple of this increment (i.e. if the increment is 0.01, order prices of 0.001 or 0.021 would be rejected).

The `base_increment` field specifies the minimum increment for the `base_currency`.

`trading_disabled` indicates whether trading is currently restricted on this product, this includes whether both new orders and order cancellations are restricted.

`cancel_only` indicates whether this product only accepts cancel requests for orders.

`post_only` indicates whether only maker orders can be placed. No orders will be matched when post\_only mode is active.

`limit_only` indicates whether this product only accepts limit orders.

Only a maximum of one of `trading_disabled`, `cancel_only`, `post_only`, `limit_only` can be true at once. If none are true, the product is trading normally.

`fx_stablecoin` indicates whether the currency pair is a Stable Pair.

`auction_mode` boolean which indicates whether or not the book is in auction mode. For more details on the auction mode see [Get product book](/api-reference/exchange-api/rest-api/products/get-product-book) describing the level 1 book which contains information pertaining to products in auction mode.

<Info>
  When limit\_only is true, matching can occur if a limit order crosses the book. Product ID will not change once assigned to a product but all other fields ares subject to change.
</Info>


## OpenAPI

````yaml GET /products
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
  /products:
    get:
      tags:
        - Products
      summary: Get all known trading pairs
      description: Gets a list of available currency pairs for trading.
      operationId: ExchangeRESTAPI_GetProducts
      parameters:
        - name: type
          in: query
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