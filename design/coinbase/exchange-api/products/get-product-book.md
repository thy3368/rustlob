> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get product book

> Get a list of open orders for a product. The amount of detail shown can be customized with the `level` parameter.

## Details

By default, only the inside (i.e., the best) bid and ask are returned. This is equivalent to a book depth of 1 level. To see a larger order book, specify the `level` query parameter.

If a level is not aggregated, all of the orders at each price are returned. Aggregated levels return only one size for each active price (as if there was only a single order for that size at the level).

## Levels

| Level | Description                                       |
| :---- | :------------------------------------------------ |
| 1     | The best bid, ask and auction info                |
| 2     | Full order book (aggregated) and auction info     |
| 3     | Full order book (non aggregated) and auction info |

**Levels 1 and 2 are aggregated**. The `size` field is the sum of the size of the orders at that `price`, and `num-orders` is the count of orders at that `price`; `size` should not be multiplied by `num-orders`.

**Level 3 is non-aggregated** and returns the entire order book.

## Auction Mode

While the book is in an auction, the L1, L2 and L3 book contain the most recent indicative quote disseminated during the auction, and `auction_mode` is set to true.

These indicative quote messages are sent on an interval basis (approximately once a second) during the collection phase of an auction and includes information about the tentative price and size affiliated with the completion.

* Opening Price - The price used for matching all the orders as the auction enters the opening state.
* Opening Size - Aggregate size of all the orders eligible for crossing Best Bid/Ask Price and Size. The anticipated BBO upon entering trading after matching has completed.

Because these indicative quote messages get disseminated on an interval basis, the values aren’t firm as changes in the book may occur between the last update and beginning the transition from auction mode to trading.

While in auction mode, the auction\_state indicates what phase the auction is in which includes:

| auction\_state |
| :------------- |
| collection     |
| opening        |
| complete       |

## Auction Details

The `collection` state indicates the auction is currently accepting orders and cancellations within the book. During this state, orders do not match and the book may appear crossed in the market data feeds.

The `opening` state indicates the book transitions towards full trading or limit only. During `opening` state any buy orders at or over the open price and any sell orders at or below the open price may cross during the opening phase. Matches in this stage are charged taker fees. Any new orders or cancels entered while in the opening phase get queued and processed when the market resumes trading.

The `complete` state indicates the dissemination of opening trades is finishing, and immediately after that the book goes into the next mode (either full trading or limit only).

The `opening` state passes by too quickly in most normal scenarios to see these states show up in the REST API.

During the `collection` state the `can_open` field indicates whether or not the book can complete the auction and enter full trading or limit only mode.

`can_open: yes` indicates the book is in a healthy state and the book can enter full trading or limit only once the auction collection state finishes.

`can_open: no` indicates the book cannot continue to full trading or limit only.

Once a book leaves auction mode — either by moving to full trading, or by being canceled by our market ops team — the book endpoint no longer shows indicative quote data and display `auction_mode` as false.

<Info>
  This request is NOT paginated. The entire book is returned in one response.
</Info>

<Info>
  Level 1 and Level 2 are recommended for polling. For the most up-to-date data, consider using the WebSocket stream. Level 3 is only recommended for users wishing to maintain a full real-time order book using the WebSocket stream. Abuse of Level 3 via polling can cause your access to be limited or blocked.
</Info>


## OpenAPI

````yaml GET /products/{product_id}/book
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
  /products/{product_id}/book:
    get:
      tags:
        - Products
      summary: Get product book
      description: >-
        Get a list of open orders for a product. The amount of detail shown can
        be customized with the `level` parameter.
      operationId: ExchangeRESTAPI_GetProductBook
      parameters:
        - name: product_id
          in: path
          required: true
          schema:
            type: string
        - name: level
          in: query
          schema:
            type: integer
            format: int32
            default: 1
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiProductBook'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
    apiProductBook:
      required:
        - asks
        - bids
        - sequence
        - time
      type: object
      properties:
        bids:
          type: array
          items:
            type: object
            properties: {}
        asks:
          type: array
          items:
            type: object
            properties: {}
        sequence:
          type: number
          format: double
        auction_mode:
          type: boolean
        auction:
          $ref: '#/components/schemas/apiAuctionBook'
        time:
          type: string
          format: date-time
      example:
        sequence: 13051505638
        bids:
          - - '6247.58'
            - '6.3578146'
            - 2
        asks:
          - - '6251.52'
            - '2'
            - 1
        time: '2021-02-12T01:09:23.334Z'
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiAuctionBook:
      required:
        - auction_state
        - best_ask_price
        - best_ask_size
        - best_bid_price
        - best_bid_size
        - open_price
        - open_size
      type: object
      properties:
        open_price:
          type: string
        open_size:
          type: string
        best_bid_price:
          type: string
        best_bid_size:
          type: string
        best_ask_price:
          type: string
        best_ask_size:
          type: string
        auction_state:
          type: string
        can_open:
          type: string
        time:
          type: string
          format: date-time
      example:
        indicative_open_price: '333.99'
        indicative_open_size: '0.193'
        indicative_bid_price: '333.98'
        indicative_bid_size: '4.39088265'
        indicative_ask_price: '333.99'
        indicative_ask_size: '25.23542881'
        auction_status: CAN_OPEN
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