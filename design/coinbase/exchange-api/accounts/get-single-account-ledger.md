> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get a single account's ledger

> Lists ledger activity for an account. This includes anything that would affect the accounts balance - transfers, trades, fees, etc.

<Warning>
  **Caution**

  If neither `start_date` nor `end_date` is set, the endpoint will return ledger activity for the past 1 day only.
</Warning>

List account activity of the API key's profile. Account activity either increases or decreases your account balance.

## API Key Permissions

This endpoint requires either the "view" or "trade" permission.

## Entry Types

Entry type indicates the reason for the account change.

| Type       | Description                                                              |
| ---------- | ------------------------------------------------------------------------ |
| transfer   | Funds moved to/from Coinbase to Coinbase Exchange                        |
| match      | Funds moved as a result of a trade                                       |
| fee        | Fee as a result of a trade                                               |
| rebate     | Fee rebate as per our [fee schedule](https://exchange.coinbase.com/fees) |
| conversion | Funds converted between fiat currency and a stablecoin                   |

## Details

If an entry is the result of a trade (match, fee), the details field contains additional information about the trade.

## Pagination

Items are paginated and sorted latest first. See [Pagination](/exchange/rest-api/pagination) for retrieving additional entries after the first page.

## Searching By Date

Searching by start and end dates are inclusive of the time provided and can be combined with before or after fields to narrow down the search to entries from a specific time range. Dates must be after Unix Epoch time and are restricted to the following formats:

* [RFC3339](https://www.rfc-editor.org/rfc/rfc3339) (i.e., `2006-01-02T15:04:05.000000Z` or `2006-01-02T15:04:05+05:30`)
* `2006-01-02`
* `2006-01-02T15:04:05`

A `400 Bad Request` error is returned for any formats that are not accepted.


## OpenAPI

````yaml GET /accounts/{account_id}/ledger
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
  /accounts/{account_id}/ledger:
    get:
      tags:
        - Accounts
      summary: Get a single account's ledger
      description: >-
        Lists ledger activity for an account. This includes anything that would
        affect the accounts balance - transfers, trades, fees, etc.
      operationId: ExchangeRESTAPI_GetAccountLedger
      parameters:
        - name: account_id
          in: path
          description: Returns list of ledger entries from this account id.
          required: true
          schema:
            type: string
        - name: start_date
          in: query
          description: >-
            Search by minimum posted date time and is inclusive of time
            provided. Valid formats are either RFC3339, date or date time and
            must be after Unix Epoch time.
          schema:
            type: string
        - name: end_date
          in: query
          description: >-
            Search by maximum posted date time and is inclusive of time
            provided. Valid formats are either RFC3339, date or date time and
            must be after Unix Epoch time.
          schema:
            type: string
        - name: before
          in: query
          description: Used for pagination. Sets start cursor to `before` id.
          schema:
            type: string
        - name: after
          in: query
          description: Used for pagination. Sets end cursor to `after` id.
          schema:
            type: string
        - name: limit
          in: query
          description: Limit on number of results to return.
          schema:
            type: integer
            format: int64
            default: 100
        - name: profile_id
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
                  $ref: '#/components/schemas/apiAccountLedgerEntry'
        '400':
          description: Bad request.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
        '401':
          description: Unauthorized.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiUnauthorizedResponse'
        '404':
          description: Not found.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
        '503':
          description: Service Unavailable.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiAccountLedgerEntry:
      required:
        - amount
        - balance
        - created_at
        - details
        - id
        - type
      type: object
      properties:
        id:
          type: string
        amount:
          type: string
        created_at:
          type: string
          format: date-time
        balance:
          type: string
        type:
          $ref: '#/components/schemas/AccountLedgerEntryAccountLedgerType'
        details:
          type: object
          additionalProperties:
            type: string
      example:
        created_at: '2019-06-11T22:11:56.382Z'
        id: '1444415179'
        amount: '3.2200000000000000'
        balance: '3.2200000000000000'
        type: transfer
        details:
          to: 6d326768-71f2-4068-99dc-7075c78f6402
          from: 20640810-6219-4d3b-95f4-5e1741dd6ea4
          profile_transfer_id: 1f854356-4923-4b10-8db1-d82f7fae8eda
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
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
    AccountLedgerEntryAccountLedgerType:
      type: string
      default: transfer
      enum:
        - transfer
        - match
        - fee
        - conversion
        - margin_interest
        - rebate
        - otc_fee
        - otc_match
        - tax_credit
        - rfq_match
        - rfq_fee
        - match_conversion
        - stake_wrap
        - conversion_fee
        - redeem
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