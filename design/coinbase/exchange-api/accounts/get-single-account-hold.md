> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get a single account's holds

> List the holds of an account that belong to the same profile as the API key. Holds are placed on an account for any active orders or pending withdraw requests. As an order is filled, the hold amount is updated. If an order is canceled, any remaining hold is removed. For withdrawals, the hold is removed after it is completed.

## Pagination

This request is paginated. See [Pagination](/exchange/rest-api/pagination) for more information.


## OpenAPI

````yaml GET /accounts/{account_id}/holds
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
  /accounts/{account_id}/holds:
    get:
      tags:
        - Accounts
      summary: Get a single account's holds
      description: >-
        List the holds of an account that belong to the same profile as the API
        key. Holds are placed on an account for any active orders or pending
        withdraw requests. As an order is filled, the hold amount is updated. If
        an order is canceled, any remaining hold is removed. For withdrawals,
        the hold is removed after it is completed.
      operationId: ExchangeRESTAPI_GetAccountHolds
      parameters:
        - name: account_id
          in: path
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
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/apiAccountHold'
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
    apiAccountHold:
      required:
        - created_at
        - id
        - ref
        - type
        - updated_at
      type: object
      properties:
        id:
          type: string
        created_at:
          type: string
          format: date-time
        updated_at:
          type: string
          format: date-time
        type:
          $ref: '#/components/schemas/AccountHoldAccountHoldType'
        ref:
          type: string
      example:
        created_at: '2020-03-11T20:48:46.622Z'
        id: c5cdd687-2d03-4a87-8dd7-c693a4bb766f
        amount: '10.0500000000000000'
        type: order
        ref: a9625b04-fc66-4999-a876-543c3684d702
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
    AccountHoldAccountHoldType:
      type: string
      default: order
      enum:
        - order
        - transfer
        - funding
        - profile_transfer
        - otc_order
        - launch_sell
        - launch_buy
        - engine_credit_operation
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