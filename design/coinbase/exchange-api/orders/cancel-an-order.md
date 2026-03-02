> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Cancel an order

> Cancel a single open order by `{id}`.

<Info>
  Cancel a previously placed order

  The order must belong to the profile that the API key belongs to. If the order had no matches during its lifetime, its record may be purged. This means the order details is not available with `GET /orders/<id>`.
</Info>

<Warning>
  To prevent a race condition when canceling an order, it is highly recommended that you specify the product id as a query string.
</Warning>

## API Key Permissions

This endpoint requires the "trade" permission.

Orders can be canceled using either the exchange assigned `id` or the client assigned `client_oid`. When using `client_oid` it must be preceded by the `client:` namespace.

## Response

A successfully cancelled order response includes:

* the order ID if the order is cancelled with the exchange assigned `id`,
* the client assigned `client_oid` if the order is cancelled with client order ID.

## Cancel Reject

If the order could not be canceled (already filled or previously canceled, etc.), then an error response indicates the reason in the `message` field.


## OpenAPI

````yaml DELETE /orders/{order_id}
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
  /orders/{order_id}:
    delete:
      tags:
        - Orders
      summary: Cancel an order
      description: Cancel a single open order by `{id}`.
      operationId: ExchangeRESTAPI_DeleteOrder
      parameters:
        - name: order_id
          in: path
          description: >-
            Orders may be canceled using either the exchange assigned id or the
            client assigned client_oid. When using client_oid it must be
            preceded by the `client:` namespace.
          required: true
          schema:
            type: string
        - name: profile_id
          in: query
          description: Cancels orders on a specific profile
          schema:
            type: string
        - name: product_id
          in: query
          description: Optional product id of order
          schema:
            type: string
      responses:
        '200':
          description: the `id` of the order that was cancelled`
          content:
            application/json:
              schema:
                type: string
                description: the `id` of the order that was cancelled`
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