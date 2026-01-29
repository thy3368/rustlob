# http proxy server

- uid->zone->ip列表->ip
- http trade[写] 根据uid 转发到axum_spot_c_server,认证

- http user data[读] 根据uid 转发到axum_spot_ud_q_server,认证

- http market data[读] 根据uid 转发到axum_spot_md_q_server，不认证


# websocket server 
http history data 根据uid 转发到axum_server


trade[写]/user_data[读]  通过user_id实现API(Command/Query)分区路由，每分区多服务器容灾， 分区实现水平扩展 通过pingora实现


api路由/仓储路由




