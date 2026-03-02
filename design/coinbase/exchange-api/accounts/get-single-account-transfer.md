> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get a single account's transfers

> Lists past withdrawals and deposits for an account.



## OpenAPI

````yaml GET /accounts/{account_id}/transfers
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
  /accounts/{account_id}/transfers:
    get:
      tags:
        - Accounts
      summary: Get a single account's transfers
      description: Lists past withdrawals and deposits for an account.
      operationId: ExchangeRESTAPI_GetAccountTransfers
      parameters:
        - name: account_id
          in: path
          description: Returns list of transfers from this account id.
          required: true
          schema:
            type: string
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
          schema:
            type: integer
            format: int64
            default: 100
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
                  $ref: '#/components/schemas/apiTransfer'
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
    apiTransfer:
      required:
        - amount
        - canceled_at
        - completed_at
        - created_at
        - currency
        - details
        - id
        - processed_at
        - type
        - user_nonce
      type: object
      properties:
        id:
          type: string
        type:
          $ref: '#/components/schemas/apiTransferType'
        created_at:
          type: string
          format: date-time
        completed_at:
          type: string
          format: date-time
        canceled_at:
          type: string
          format: date-time
        processed_at:
          type: string
          format: date-time
        amount:
          type: string
        details:
          type: object
          additionalProperties:
            type: string
        user_nonce:
          type: string
          format: int64
        currency:
          type: string
      example:
        id: 19ac524d-8827-4246-a1b2-18dc5ca9472c
        type: withdraw
        created_at: '2020-03-12T00:14:12.397Z'
        completed_at: '2020-03-12T00:14:13.021Z'
        amount: '1.00000000'
        details:
          coinbase_account_id: 2b760113-fbba-5600-ac74-36482c130768
          coinbase_transaction_id: 5e697ed49f8417148f3366ea
          coinbase_payment_method_id: ''
        currency: USD
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
    apiTransferType:
      type: string
      default: deposit
      enum:
        - deposit
        - withdraw
        - internal_deposit
        - internal_withdraw
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