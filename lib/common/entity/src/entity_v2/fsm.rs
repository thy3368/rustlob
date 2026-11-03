/// 与业务无关的实体有限状态机契约。
///
/// 实现应先完成领域校验和派生，再提交实体状态；失败时不得留下部分变更。
pub trait Fsm {
    type State;
    type Event;
    type Output;
    type Error;

    fn transition(
        &mut self,
        event: Self::Event,
    ) -> Result<(Self::State, Self::Output), Self::Error>;

    fn apply(&mut self, event: Self::Event) -> Result<(), Self::Error> {
        let _ = self.transition(event)?;

        Ok(())
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
        type Event = Event;
        type Output = u64;
        type Error = InvalidTransition;

        fn transition(
            &mut self,
            event: Self::Event,
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

    #[test]
    fn apply_ignores_transition_output() {
        let mut machine = Machine { state: State::Ready, ver: 0 };

        assert_eq!(machine.apply(Event::Inc), Ok(()));
        assert_eq!((machine.state, machine.ver), (State::Done, 1));
    }
}
