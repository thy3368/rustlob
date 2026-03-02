> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get conversion fee rates

> Gets a list of current conversion fee rates and trailing 30 day volume by currency pair



## OpenAPI

````yaml GET /conversions/fees
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
  /conversions/fees:
    get:
      tags:
        - Conversions
      summary: Get conversion fee rates
      description: >-
        Gets a list of current conversion fee rates and trailing 30 day volume
        by currency pair
      operationId: ExchangeRESTAPI_GetConversionFees
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/apiConversionFee'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiConversionFee:
      required:
        - available_credit
        - from_currency
        - to_currency
      type: object
      properties:
        from_currency:
          type: string
        to_currency:
          type: string
        min_fee_rate:
          type: string
        thirty_day_net_volume:
          type: string
        fee_tiers:
          type: array
          items:
            $ref: '#/components/schemas/apiConversionFeeTier'
        available_credit:
          type: string
      example:
        from_currency: USDC
        to_currency: USD
        fee_rate: '0.001'
        thirty_day_volume: '1000000.00000000'
        available_credit: '1000000.00000000'
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiConversionFeeTier:
      required:
        - fee_rate
        - from_amount
        - to_amount
      type: object
      properties:
        from_amount:
          type: string
        to_amount:
          type: string
        fee_rate:
          type: string
      example:
        from_amount: '0'
        to_amount: '75000000'
        fee_rate: '0.001'
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