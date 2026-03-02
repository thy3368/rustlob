> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Cancel all orders

> With best effort, cancel all open orders. This may require you to make the request multiple times until all of the open orders are deleted.

## API Key Permissions

This endpoint requires the "trade" permission.

## Examples

| Example                      | Response                                 |
| :--------------------------- | :--------------------------------------- |
| `/orders?product_id=FOO-BAR` | (404) ProductNotFound                    |
| `/orders?product_id=BtC-uSd` | (200) Cancel all orders for BTC-USD      |
| `/orders?Product_id=BTC-USD` | (400) Return BadRequest Error            |
| `/orders`                    | (200) Cancel all orders for all products |


## OpenAPI

````yaml DELETE /orders
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
  /orders:
    delete:
      tags:
        - Orders
      summary: Cancel all orders
      description: >-
        With best effort, cancel all open orders. This may require you to make
        the request multiple times until all of the open orders are deleted.
      operationId: ExchangeRESTAPI_DeleteOrders
      parameters:
        - name: profile_id
          in: query
          description: Cancels orders on a specific profile
          schema:
            type: string
        - name: product_id
          in: query
          description: Cancels orders on a specific product only
          schema:
            type: string
      responses:
        '200':
          description: A list of the `id`s of open orders that were successfully cancelled
          content:
            application/json:
              schema:
                type: array
                description: >-
                  A list of the `id`s of open orders that were successfully
                  cancelled
                items:
                  type: string
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