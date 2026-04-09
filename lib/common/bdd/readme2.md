
1， 定义use case 签名 define_use_case, 

用


2，定义bdd验收用例 define_bdd_case_4_use_case

一个use case的
bdd 应该是一个矩阵


scenario 对应 use case

given 对应pre state

when 对应指定的特定的command


case 覆盖  given 穷举与 when的交集。



#[bdd_test(
feature = "购物车管理",
scenario = "添加重复商品",
given(购物车中已有商品A),
when = "再次添加相同的商品A",
then(商品数量应增加, 总价应相应计算),
tags(cart, duplicate),
priority = "3"
)]


3, run_bdd_case