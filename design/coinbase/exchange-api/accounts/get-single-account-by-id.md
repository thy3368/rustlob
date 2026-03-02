> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get a single account by id

> Information for a single account. Use this endpoint when you know the account_id. API key must belong to the same profile as the account.

## API Key Permissions

This endpoint requires either the "view" or "trade" permission.


## OpenAPI

````yaml GET  /accounts/{account_id}
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
  /accounts/{account_id}:
    get:
      tags:
        - Accounts
      summary: Get a single account by id
      description: >-
        Information for a single account. Use this endpoint when you know the
        account_id. API key must belong to the same profile as the account.
      operationId: ExchangeRESTAPI_GetAccount
      parameters:
        - name: account_id
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
                $ref: '#/components/schemas/apiAccount'
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
    apiAccount:
      required:
        - available
        - balance
        - currency
        - hold
        - id
        - profile_id
        - trading_enabled
      type: object
      properties:
        id:
          type: string
        currency:
          type: string
        balance:
          type: string
        hold:
          type: string
        available:
          type: string
        profile_id:
          type: string
        trading_enabled:
          type: boolean
        pending_deposit:
          type: string
          description: Amount in pending deposits transfers.
        display_name:
          type: string
      example:
        id: 7fd0abc0-e5ad-4cbb-8d54-f2b3f43364da
        currency: USD
        balance: '0.0000000000000000'
        hold: '0.0000000000000000'
        available: '0'
        profile_id: 8058d771-2d88-4f0f-ab6e-299c153d4308
        trading_enabled: true
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