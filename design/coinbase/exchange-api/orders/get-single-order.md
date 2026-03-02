> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get single order

> Get a single order by `id`.

## API Key Permissions

This endpoint requires either the "view" or "trade" permission.

Orders can be queried using either the exchange assigned `id` or the client assigned `client_oid`. When using `client_oid` it must be preceded by the `client:` namespace.

If the order is canceled, and if the order had no matches, the response might return the status code `404`.

<Info>
  Open orders can change state between the request and the response depending on market conditions.
</Info>


## OpenAPI

````yaml GET /orders/{order_id}
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
  /orders/{order_id}:
    get:
      tags:
        - Orders
      summary: Get single order
      description: Get a single order by `id`.
      operationId: ExchangeRESTAPI_GetOrder
      parameters:
        - name: order_id
          in: path
          description: >-
            `order_id` is either the exchange assigned id or the client assigned
            client_oid. When using client_oid it must be preceded by the client:
            namespace.
          required: true
          schema:
            type: string
        - name: market_type
          in: query
          description: Market type which the order was traded in.
          schema:
            type: string
            default: spot
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiOrder'
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
    apiOrder:
      required:
        - created_at
        - fill_fees
        - filled_size
        - id
        - post_only
        - product_id
        - settled
        - side
        - status
        - type
      type: object
      properties:
        id:
          type: string
          description: uuid
        price:
          type: string
          description: price per unit of base currency
        size:
          type: string
          description: amount of base currency to buy/sell
        product_id:
          type: string
          description: book the order was placed on
        profile_id:
          type: string
          description: profile_id that placed the order
        side:
          $ref: '#/components/schemas/apiOrderSide'
        funds:
          type: string
          description: amount of quote currency to spend (for market orders)
        specified_funds:
          type: string
          description: funds with fees
        type:
          $ref: '#/components/schemas/apiOrderType'
        time_in_force:
          $ref: '#/components/schemas/apiOrderTimeInForce'
        expire_time:
          type: string
          description: timestamp at which order expires
          format: date-time
        post_only:
          type: boolean
          description: if true, forces order to be `maker` only
        created_at:
          type: string
          description: timestamp at which order was placed
          format: date-time
        done_at:
          type: string
          description: timestamp at which order was done
          format: date-time
        done_reason:
          type: string
          description: reason order was done (filled, rejected, or otherwise)
        reject_reason:
          type: string
          description: reason order was rejected by engine
        fill_fees:
          type: string
          description: fees paid on current filled amount
        filled_size:
          type: string
          description: amount (in base currency) of the order that has been filled
        executed_value:
          type: string
        status:
          $ref: '#/components/schemas/apiOrderStatus'
        settled:
          type: boolean
          description: true if funds have been exchanged and settled
        stop:
          $ref: '#/components/schemas/apiOrderStop'
        stop_price:
          type: string
          description: price (in quote currency) at which to execute the order
        funding_amount:
          type: string
        client_oid:
          type: string
          description: client order id
        market_type:
          type: string
          description: market type where order was traded
        max_floor:
          type: string
          description: maximum visible quantity for iceberg order
        secondary_order_id:
          type: string
          description: order id for the visible order for iceberg order
        stop_limit_price:
          type: string
          description: stop limit price for TPSL order
      example:
        id: a9625b04-fc66-4999-a876-543c3684d702
        price: '10.00000000'
        size: '1.00000000'
        product_id: BTC-USD
        profile_id: 8058d771-2d88-4f0f-ab6e-299c153d4308
        side: buy
        type: limit
        time_in_force: GTC
        post_only: true
        max_floor: '4'
        created_at: '2020-03-11T20:48:46.622Z'
        fill_fees: '0.0000000000000000'
        filled_size: '0.00000000'
        executed_value: '0.0000000000000000'
        status: open
        settled: false
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
    apiOrderSide:
      type: string
      default: buy
      enum:
        - buy
        - sell
    apiOrderType:
      type: string
      default: limit
      enum:
        - limit
        - market
        - stop
    apiOrderTimeInForce:
      type: string
      default: GTC
      enum:
        - GTC
        - GTT
        - IOC
        - FOK
    apiOrderStatus:
      type: string
      default: open
      enum:
        - open
        - pending
        - rejected
        - done
        - active
        - received
        - all
    apiOrderStop:
      type: string
      default: loss
      enum:
        - loss
        - entry
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