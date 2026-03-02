> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get all known currencies

> Gets a list of all known currencies.
 Note: Not all currencies may be currently in use for trading.

<Info>
  **Info**

  Not all currencies may be currently in use for trading.
</Info>


## OpenAPI

````yaml GET /currencies
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
  /currencies:
    get:
      tags:
        - Currencies
      summary: Get all known currencies
      description: |-
        Gets a list of all known currencies.
         Note: Not all currencies may be currently in use for trading.
      operationId: ExchangeRESTAPI_GetCurrencies
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/apiCurrency'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
      security: []
components:
  schemas:
    apiCurrency:
      required:
        - details
        - id
        - max_precision
        - min_size
        - name
        - status
      type: object
      properties:
        id:
          type: string
        name:
          type: string
        min_size:
          type: string
        status:
          type: string
        message:
          type: string
        max_precision:
          type: string
        convertible_to:
          type: array
          items:
            type: string
        details:
          $ref: '#/components/schemas/apiCurrencyDetails'
        default_network:
          type: string
        supported_networks:
          type: array
          items:
            $ref: '#/components/schemas/apiNetwork'
        display_name:
          type: string
      example:
        id: USD
        name: United States Dollar
        min_size: '0.01'
        max_precision: '0.01'
        status: online
        details:
          type: fiat
          symbol: $
          sort_order: 1
          push_payment_methods:
            - bank_wire
            - fedwire
            - swift_bank_account
            - intra_bank_account
          display_name: US Dollar
          group_types:
            - fiat
            - usd
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiCurrencyDetails:
      type: object
      properties:
        type:
          $ref: '#/components/schemas/apiCurrencyType'
        symbol:
          type: string
        network_confirmations:
          type: integer
          format: int32
        sort_order:
          type: integer
          format: int32
        crypto_address_link:
          type: string
        crypto_transaction_link:
          type: string
        push_payment_methods:
          type: array
          items:
            type: string
        group_types:
          type: array
          items:
            type: string
        display_name:
          type: string
        processing_time_seconds:
          type: number
          format: float
        min_withdrawal_amount:
          type: number
          format: double
        max_withdrawal_amount:
          type: number
          format: double
    apiNetwork:
      type: object
      properties:
        id:
          type: string
        name:
          type: string
        status:
          type: string
        contract_address:
          type: string
        crypto_address_link:
          type: string
        crypto_transaction_link:
          type: string
        min_withdrawal_amount:
          type: number
          format: double
        max_withdrawal_amount:
          type: number
          format: double
        network_confirmations:
          type: integer
          format: int32
        processing_time_seconds:
          type: integer
          format: int32
        destination_tag_regex:
          type: string
        is_evm_network:
          type: boolean
          description: Indicates whether this network is an EVM-compatible blockchain.
    apiCurrencyType:
      type: string
      default: crypto
      enum:
        - crypto
        - fiat
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