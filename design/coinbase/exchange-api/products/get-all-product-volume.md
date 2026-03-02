> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get all product volume

> Gets 30day and 24hour volume for all products and market types



## OpenAPI

````yaml GET  /products/volume-summary
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
  /products/volume-summary:
    get:
      tags:
        - Products
      summary: Get all product volume
      description: Gets 30day and 24hour volume for all products and market types
      operationId: ExchangeRESTAPI_GetProductsVolume
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/apiProductVolume'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
    apiProductVolume:
      required:
        - base_currency
        - conversion_volume_24hour
        - conversion_volume_30day
        - display_name
        - id
        - market_types
        - quote_currency
        - rfq_volume_24hour
        - rfq_volume_30day
        - spot_volume_24hour
        - spot_volume_30day
      type: object
      properties:
        id:
          type: string
        base_currency:
          type: string
        quote_currency:
          type: string
        display_name:
          type: string
        market_types:
          type: array
          items:
            type: object
            properties: {}
        spot_volume_24hour:
          type: string
        spot_volume_30day:
          type: string
        rfq_volume_24hour:
          type: string
        rfq_volume_30day:
          type: string
        conversion_volume_24hour:
          type: string
        conversion_volume_30day:
          type: string
      example:
        - id: GALA-XYO
          base_currency: GALA
          quote_currency: XYO
          display_name: GALA-XYO
          market_types:
            - rfq
          spot_volume_24hour: ''
          spot_volume_30day: ''
          rfq_volume_24hour: '1232.2342'
          rfq_volume_30day: '2453232.2342'
          conversion_volume_24hour: '0'
          conversion_volume_30day: '0'
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