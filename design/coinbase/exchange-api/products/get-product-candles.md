> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get product candles

> Historic rates for a product. Rates are returned in grouped buckets. Candle schema is of the form `[timestamp, price_low, price_high, price_open, price_close]`

<Warning>
  * Historical rate data may be incomplete. No data is published for intervals where there are no ticks.
  * Historical rates should *not* be polled frequently. For real-time info, use the trade and book endpoints in conjunction with the WebSocket feed.
</Warning>

## Time Range

If the `start` or `end` fields are not provided, both fields are ignored. If a custom time range is not declared, then one ending now is selected.

The `granularity` field must be one of the following "second" values: `{60, 300, 900, 3600, 21600, 86400}`, or your request is rejected. These values correspond to timeslices representing one minute, five minutes, fifteen minutes, one hour, six hours, and one day, respectively.

If data points are readily available, your response may contain as many as `300` candles and some of those candles may precede your declared `start` value.

## Max Candles

<Info>
  The maximum number of data points for a single request is `300` candles.
</Info>

If your selection of start/end time and granularity results in more than `300` data points, your request is rejected. To retrieve fine granularity data over a larger time range, you must make multiple requests with new start/end ranges.

## Response Items

Each bucket is an array of the following information:

* `time` bucket start time
* `low` lowest price during the bucket interval
* `high` highest price during the bucket interval
* `open` opening price (first trade) in the bucket interval
* `close` closing price (last trade) in the bucket interval
* `volume` volume of trading activity during the bucket interval


## OpenAPI

````yaml GET /products/{product_id}/candles
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
  /products/{product_id}/candles:
    get:
      tags:
        - Products
      summary: Get product candles
      description: >-
        Historic rates for a product. Rates are returned in grouped buckets.
        Candle schema is of the form `[timestamp, price_low, price_high,
        price_open, price_close]`
      operationId: ExchangeRESTAPI_GetProductCandles
      parameters:
        - name: product_id
          in: path
          required: true
          schema:
            type: string
        - name: granularity
          in: query
          schema:
            type: string
        - name: start
          in: query
          description: Timestamp for starting range of aggregations
          schema:
            type: string
        - name: end
          in: query
          description: Timestamp for ending range of aggregations
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
                  type: object
                  properties: {}
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
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