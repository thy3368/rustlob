
1， 定义use case 签名 define_use_case, 


用 /Users/hongyaotang/src/rustlob/lib/common/base_types/src/handler/handler_update2.rs 的CmdHandlerForUpdate2 定义command handler


2，定义bdd验收用例 define_bdd_case_4_use_case



规则：

- scenario 对应 handler name

- given 对应GivenStateSet

- when 对应指定的特定的command

- then 对应ThenStateSet

- bdd_case 覆盖  given穷举与 when的穷举的交集。


    type Command;
    type Reply;
    type GivenStateSet;
    type ThenStateSet: DomainEventSet;
    type Error;




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