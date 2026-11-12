/// 与业务无关的实体有限状态机契约。
///
/// 实现应先完成领域校验和派生，再提交实体状态；失败时不得留下部分变更。
pub trait Fsm {
    type State;

    ///内部业务事件，不可变唯一事实，驱动状态变化的唯一口子
    type BizEvent;
    type Output;
    type Error;

    ///不建议直接调用

    fn transition(
        &mut self,
        event: Self::BizEvent,
    ) -> Result<(Self::State, Self::Output), Self::Error>;

    ///直接调用
    fn apply(&mut self, event: Self::BizEvent) -> Result<(Self::State, Self::Output), Self::Error> {
        self.transition(event)
    }
}

#[cfg(test)]
mod tests {
    use super::Fsm;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum State {
        Ready,
        Done,
    }

    #[derive(Debug, Clone, Copy)]
    enum Event {
        Inc,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct InvalidTransition;

    struct Machine {
        pub state: State,
        pub ver: u64,
    }

    impl Fsm for Machine {
        type State = State;
        type BizEvent = Event;
        type Output = u64;
        type Error = InvalidTransition;

        fn transition(
            &mut self,
            event: Self::BizEvent,
        ) -> Result<(Self::State, Self::Output), Self::Error> {
            match (self.state, event) {
                (State::Ready, Event::Inc) => {
                    //先做判断 最后改
                    self.ver = 1;
                    self.state = State::Done;
                    Ok((self.state, self.ver))
                }
                (State::Done, Event::Inc) => Err(InvalidTransition),
            }
        }
    }

    #[test]
    fn transition_commits_successfully_and_apply_discards_output() {
        let mut machine = Machine { state: State::Ready, ver: 0 };
        assert_eq!(machine.transition(Event::Inc), Ok((State::Done, 1)));
        assert_eq!(machine.state, State::Done);

        assert_eq!(machine.apply(Event::Inc), Err(InvalidTransition));
        assert_eq!(machine.state, State::Done);
    }
}
