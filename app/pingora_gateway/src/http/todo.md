# http proxy server

- uid->zone->ip列表->ip
- http trade[写] 根据uid 转发到axum_spot_c_server,认证

- http user data[读] 根据uid 转发到axum_spot_ud_q_server,认证

- http market data[读] 根据uid 转发到axum_spot_md_q_server，不认证


# websocket server 
http history data 根据uid 转发到axum_server