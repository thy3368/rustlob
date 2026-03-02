> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Open new loan

> This API triggers a loan open request. Funding is not necessarily instantaneous and there is no SLA. You are notified when funds have settled in your Exchange account. Loan open requests, once initiated, cannot be canceled.

<Info>
  **Coinbase Exchange Loans Program**

  See [Coinbase Exchange Loans Program](https://coinbase.bynder.com/m/47c334b9a63ed3e4/original/exchange-Loans-Program.pdf) for program details including qualification criteria and sample terms.
</Info>

<Warning>
  **Caution**

  The lending rate limit is [10 RPS per profile](/exchange/rest-api/rate-limits).
</Warning>


## OpenAPI

````yaml POST /loans/open
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
  /loans/open:
    post:
      tags:
        - Loans
      summary: Open new loan
      description: >-
        This API triggers a loan open request. Funding is not necessarily
        instantaneous and there is no SLA. You are notified when funds have
        settled in your Exchange account. Loan open requests, once initiated,
        cannot be canceled.
      operationId: ExchangeRESTAPI_OpenLoan
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/apiOpenLoanRequest'
        required: true
      responses:
        '200':
          description: A successful response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiOpenLoanResponse'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiOpenLoanRequest:
      type: object
      properties:
        loan_id:
          type: string
        currency:
          type: string
        native_amount:
          type: string
        interest_rate:
          type: string
        term_start_date:
          type: string
          format: date-time
        term_end_date:
          type: string
          format: date-time
        profile_id:
          type: string
    apiOpenLoanResponse:
      type: object
      properties:
        loan:
          $ref: '#/components/schemas/apiLoan'
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiLoan:
      type: object
      properties:
        id:
          type: string
        currency:
          type: string
        principal_amount:
          type: string
        outstanding_principal_amount:
          type: string
        interest_rate:
          type: string
        interest_currency:
          type: string
        status:
          $ref: '#/components/schemas/apiLoanStatus'
        effective_at:
          type: string
          format: date-time
        closed_at:
          type: string
          format: date-time
        term_start_date:
          type: string
          format: date-time
        term_end_date:
          type: string
          format: date-time
    apiLoanStatus:
      type: string
      default: loan_pending
      enum:
        - loan_pending
        - loan_active
        - loan_canceled
        - loan_closed
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