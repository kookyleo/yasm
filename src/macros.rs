/// Internal helper macro - generates common parts of state machine
#[macro_export]
#[doc(hidden)] // Hide internal macro
macro_rules! __define_state_machine_common {
    (
        $name:ident,
        { $($state:ident),* },
        { $($input:ident),* },
        $initial:ident,
        { $( $from:ident + $inp:ident => $to:ident ),* }
    ) => {
        /// State enumeration type
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        pub enum State {
            $($state),*
        }

        /// Input enumeration type
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        pub enum Input {
            $($input),*
        }

        impl std::fmt::Display for State {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(State::$state => write!(f, stringify!($state)),)*
                }
            }
        }

        impl std::fmt::Display for Input {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Input::$input => write!(f, stringify!($input)),)*
                }
            }
        }

        /// State machine struct
        pub struct $name;

        impl $crate::StateMachine for $name {
            type State = State;
            type Input = Input;

            fn states() -> Vec<Self::State> {
                vec![$(State::$state),*]
            }

            fn inputs() -> Vec<Self::Input> {
                vec![$(Input::$input),*]
            }

            fn initial_state() -> Self::State {
                State::$initial
            }

            fn state_name(state: &Self::State) -> String {
                format!("{:?}", state)
            }

            fn input_name(input: &Self::Input) -> String {
                format!("{:?}", input)
            }

            fn valid_inputs(state: &Self::State) -> Vec<Self::Input> {
                let mut inputs = Vec::new();
                $(
                    if matches!(state, State::$from) {
                        inputs.push(Input::$inp);
                    }
                )*
                inputs
            }

            /// Deterministic state transition implementation
            fn next_state(state: &Self::State, input: &Self::Input) -> Option<Self::State> {
                #[allow(unreachable_patterns)]
                match (state, input) {
                    $(
                        (State::$from, Input::$inp) => Some(State::$to),
                    )*
                    _ => None,
                }
            }
        }
    };
}

/// Serde support helper macro
#[macro_export]
#[doc(hidden)]
macro_rules! __define_state_machine_serde {
    ({ $($state:ident),* }, { $($input:ident),* }) => {
        impl serde::Serialize for State {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(State::$state => serializer.serialize_str(stringify!($state)),)*
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for State {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match s.as_str() {
                    $(stringify!($state) => Ok(State::$state),)*
                    _ => Err(serde::de::Error::custom(format!("Unknown state: {}", s))),
                }
            }
        }

        impl serde::Serialize for Input {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(Input::$input => serializer.serialize_str(stringify!($input)),)*
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for Input {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                match s.as_str() {
                    $(stringify!($input) => Ok(Input::$input),)*
                    _ => Err(serde::de::Error::custom(format!("Unknown input: {}", s))),
                }
            }
        }
    };
}

/// Macro for defining deterministic state machines - serde-enabled version
///
/// This macro is used to quickly define deterministic state machines where each state+input
/// combination can have at most one next state.
///
/// # Syntax
/// ```rust
/// use yasm::define_state_machine;
/// define_state_machine! {
///     name: MyStateMachine,
///     states: { State1, State2, State3 },
///     inputs: { Input1, Input2 },
///     initial: State1,
///     transitions: {
///         State1 + Input1 => State2,
///         State2 + Input2 => State3,
///     }
/// }
/// ```
///
/// # Parameters
/// - `name`: Name of the state machine struct
/// - `states`: List of all possible states
/// - `inputs`: List of all possible inputs
/// - `initial`: Initial state
/// - `transitions`: State transition rules in the format `from_state + input => to_state`
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! define_state_machine {
    (
        name: $name:ident,
        states: { $($state:ident),* $(,)? },
        inputs: { $($input:ident),* $(,)? },
        initial: $initial:ident,
        transitions: {
            $(
                $from:ident + $inp:ident => $to:ident
            ),* $(,)?
        }
    ) => {
        // Call common part
        $crate::__define_state_machine_common!(
            $name,
            { $($state),* },
            { $($input),* },
            $initial,
            { $( $from + $inp => $to ),* }
        );

        // Add serde support
        $crate::__define_state_machine_serde!(
            { $($state),* },
            { $($input),* }
        );
    };
}

/// Macro for defining deterministic state machines - non-serde version
///
/// This macro is used to quickly define deterministic state machines where each state+input
/// combination can have at most one next state.
///
/// # Syntax
/// ```rust
/// use yasm::define_state_machine;
/// define_state_machine! {
///     name: MyStateMachine,
///     states: { State1, State2, State3 },
///     inputs: { Input1, Input2 },
///     initial: State1,
///     transitions: {
///         State1 + Input1 => State2,
///         State2 + Input2 => State3,
///     }
/// }
/// ```
///
/// # Parameters
/// - `name`: Name of the state machine struct
/// - `states`: List of all possible states
/// - `inputs`: List of all possible inputs
/// - `initial`: Initial state
/// - `transitions`: State transition rules in the format `from_state + input => to_state`
#[cfg(not(feature = "serde"))]
#[macro_export]
macro_rules! define_state_machine {
    (
        name: $name:ident,
        states: { $($state:ident),* $(,)? },
        inputs: { $($input:ident),* $(,)? },
        initial: $initial:ident,
        transitions: {
            $(
                $from:ident + $inp:ident => $to:ident
            ),* $(,)?
        }
    ) => {
        // Call common part
        $crate::__define_state_machine_common!(
            $name,
            { $($state),* },
            { $($input),* },
            $initial,
            { $( $from + $inp => $to ),* }
        );
    };
}
