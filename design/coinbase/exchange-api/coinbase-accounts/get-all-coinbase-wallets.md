> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get all Coinbase wallets

> Gets all the user's available Coinbase wallets (These are the wallets/accounts that are used for buying and selling on www.coinbase.com)



## OpenAPI

````yaml GET /coinbase-accounts
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
  /coinbase-accounts:
    get:
      tags:
        - Coinbase accounts
      summary: Get all Coinbase wallets
      description: >-
        Gets all the user's available Coinbase wallets (These are the
        wallets/accounts that are used for buying and selling on
        www.coinbase.com)
      operationId: ExchangeRESTAPI_GetCoinbaseAccounts
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/apiCoinbaseAccount'
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
    apiCoinbaseAccount:
      required:
        - active
        - balance
        - currency
        - hold_balance
        - hold_currency
        - id
        - name
        - primary
        - type
      type: object
      properties:
        id:
          type: string
        name:
          type: string
        balance:
          type: string
        currency:
          type: string
        type:
          $ref: '#/components/schemas/CoinbaseAccountCoinbaseAccountType'
        primary:
          type: boolean
        active:
          type: boolean
        hold_balance:
          type: string
        hold_currency:
          type: string
        destination_tag_name:
          type: string
        destination_tag_regex:
          type: string
      example:
        available_on_consumer: true
        hold_balance: '0.00'
        id: OXT
        hold_currency: USD
        balance: '0.00000000'
        currency: OXT
        primary: false
        name: OXT Wallet
        type: wallet
        active: true
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
    CoinbaseAccountCoinbaseAccountType:
      type: string
      default: wallet
      enum:
        - wallet
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