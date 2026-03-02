> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get all orders

> List your current open orders. Only open or un-settled orders are returned by default. As soon as an order is no longer open and settled, it will no longer appear in the default request. Open orders may change state between the request and the response depending on market conditions.

## Pending Orders

Orders with a "pending" status have fewer fields in the response.

* Pending limit orders do not have `stp`, `time_in_force`, `expire_time`, and `post_only`.
* Pending market orders have the same fields as a pending limit order minus `price` and `size`, and no market specific fields (`funds`, `specified_funds`).
* Pending stop orders have the same fields as a pending limit order and no stop specific fields (`stop`, `stop_price`).

| Order Type           | Does Not Have These Fields                                                                      |
| :------------------- | :---------------------------------------------------------------------------------------------- |
| Pending Limit Order  | `stp`, `time_in_force`, `expire_time`, `post_only`                                              |
| Pending Market Order | `stp`, `time_in_force`, `expire_time`, `post_only`, `price`, `size`, `funds`, `specified_funds` |
| Pending Stop Order   | `stp`, `time_in_force`, `expire_time`, `post_only`, `stop`, `stop_price`                        |

## API Key Permissions

This endpoint requires either the "view" or "trade" permission.

<Tip>
  To specify multiple statuses, use the status query argument multiple times: `/orders?status=done&status=pending`.
</Tip>

## Order Status and Settlement

Orders which are no longer resting on the order book, are marked with the `done` status. There is a small window between an order being `done` and `settled`. An order is settled when all of the fills have settled and the remaining holds (if any) have been removed.

## Polling

For high-volume trading it is strongly recommended that you maintain your own list of open orders and use one of the streaming market data feeds to keep it updated. You should poll the open orders endpoint once when you start trading to obtain the current state of any open orders.

`executed_value` is the cumulative match `size` \* `price` and is only present for orders placed after 2016-05-20.

<Info>
  Open orders can change state between the request and the response depending on market conditions.
</Info>

## Pagination

This request is paginated. See [Pagination](/exchange/rest-api/pagination) for more information.


## OpenAPI

````yaml GET /orders
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
  /orders:
    get:
      tags:
        - Orders
      summary: Get all orders
      description: >-
        List your current open orders. Only open or un-settled orders are
        returned by default. As soon as an order is no longer open and settled,
        it will no longer appear in the default request. Open orders may change
        state between the request and the response depending on market
        conditions.
      operationId: ExchangeRESTAPI_GetOrders
      parameters:
        - name: profile_id
          in: query
          description: Filter results by a specific profile_id
          schema:
            type: string
        - name: product_id
          in: query
          description: Filter results by a specific product_id
          schema:
            type: string
        - name: sortedBy
          in: query
          description: Sort criteria for results.
          schema:
            type: string
            default: created_at
            enum:
              - created_at
              - price
              - size
              - order_id
              - side
              - type
        - name: sorting
          in: query
          description: Ascending or descending order, by `sortedBy`
          schema:
            type: string
            default: desc
            enum:
              - desc
              - asc
        - name: start_date
          in: query
          description: Filter results by minimum posted date
          schema:
            type: string
            format: date-time
        - name: end_date
          in: query
          description: Filter results by maximum posted date
          schema:
            type: string
            format: date-time
        - name: before
          in: query
          description: Used for pagination. Sets start cursor to `before` date.
          schema:
            type: string
        - name: after
          in: query
          description: Used for pagination. Sets end cursor to `after` date.
          schema:
            type: string
        - name: limit
          in: query
          description: Limit on number of results to return.
          required: true
          schema:
            type: integer
            format: int64
            default: 100
        - name: status
          in: query
          description: Array with order statuses to filter by.
          required: true
          style: form
          explode: true
          schema:
            type: array
            items:
              type: string
              enum:
                - open
                - pending
                - rejected
                - done
                - active
                - received
                - all
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
                type: array
                items:
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