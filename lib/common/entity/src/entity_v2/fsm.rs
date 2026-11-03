/// 与业务无关的实体有限状态机契约。
///
/// `transition` 调用业务方法并变更状态，怎么避免改了部份又返回错误的情况。先做各种领域检查，最后完成数据修改
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
        self.transition(event)?;

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
    fn transition_is_pure_and_apply_commits_only_successful_transitions() {
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
