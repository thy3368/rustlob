> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get repayment preview XM

> Preview the results of a loan principal repayment for a cross margin user.

Preview the results of a loan principal repayment for a cross margin user. This endpoint shows the before and after state of your lending overview when repaying a loan.

<Info>
  **Repayment Preview for Cross Margin (XM) Users**

  Use this endpoint to preview how repaying a loan would affect your cross margin account's margin requirements, account equity, and other lending metrics before actually making the repayment.
</Info>

<Warning>
  **Caution**

  The lending rate limit is [10 RPS per profile](/exchange/rest-api/rate-limits).
</Warning>


## OpenAPI

````yaml GET /loans/repayment-preview-xm
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
  /loans/repayment-preview-xm:
    get:
      tags:
        - Loans
      summary: Get repayment preview XM
      description: >-
        Preview the results of a loan principal repayment for a cross margin
        user.
      operationId: ExchangeRESTAPI_GetRepaymentPreviewXM
      parameters:
        - name: loan_id
          in: query
          required: true
          schema:
            type: string
        - name: currency
          in: query
          required: true
          schema:
            type: string
        - name: native_amount
          in: query
          required: true
          schema:
            type: string
      responses:
        '200':
          description: A successful response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiGetPreviewXMResponse'
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
    apiGetPreviewXMResponse:
      type: object
      properties:
        before:
          $ref: '#/components/schemas/apiLendingOverviewXM'
        after:
          $ref: '#/components/schemas/apiLendingOverviewXM'
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
    apiLendingOverviewXM:
      type: object
      properties:
        open_loan_value:
          type: string
          description: Total value of open loans
        available_to_borrow:
          type: string
          description: Amount available to borrow
        withdrawal_restricted:
          type: boolean
          description: Whether withdrawals are restricted
        credit_limit_value:
          type: string
          description: Total credit limit
        available_credit_value:
          type: string
          description: Available credit
        pending_loan_value:
          type: string
          description: Value of pending loans
        margin_requirement:
          type: string
          description: Current margin requirement
        account_equity:
          type: string
          description: Total account equity
        margin_excess_shortfall:
          type: string
          description: Margin excess or shortfall amount
        consumed_credit:
          type: string
          description: Amount of credit consumed
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