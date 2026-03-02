> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get product stats

> Gets 30day and 24hour stats for a product.

<Info>
  The`volume` property is in base currency units. Properties `open`, `high`, `low` are in quote currency units.
</Info>


## OpenAPI

````yaml GET /products/{product_id}/stats
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
  /products/{product_id}/stats:
    get:
      tags:
        - Products
      summary: Get product stats
      description: Gets 30day and 24hour stats for a product.
      operationId: ExchangeRESTAPI_GetProductStats
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
                $ref: '#/components/schemas/apiProductStats'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
    apiProductStats:
      required:
        - high
        - last
        - low
        - open
        - volume
      type: object
      properties:
        open:
          type: string
        high:
          type: string
        low:
          type: string
        last:
          type: string
        volume:
          type: string
        volume_30day:
          type: string
        rfq_volume_24hour:
          type: string
        rfq_volume_30day:
          type: string
        conversions_volume_24hour:
          type: string
        conversions_volume_30day:
          type: string
      example:
        open: '5414.18000000'
        high: '6441.37000000'
        low: '5261.69000000'
        volume: '53687.76764233'
        last: '6250.02000000'
        volume_30day: '786763.72930864'
        rfq_volume_24hour: '78.23'
        conversions_volume_24hour: '0.000000'
        rfq_volume_30day: '0.000000'
        conversions_volume_30day: '0.000000'
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