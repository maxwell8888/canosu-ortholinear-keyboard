use keyberon::action::{k, Action, Action::*};
use keyberon::key_code::KeyCode;
use keyberon::key_code::KeyCode::*;

use crate::{NUM_COLS, NUM_LAYERS, NUM_ROWS};
#[allow(unused_macros)]

// Shift + KeyCode
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomActions {
    Bootloader,
}

#[allow(dead_code)]
const BOOTLOADER: Action<CustomActions> = Action::Custom(CustomActions::Bootloader);

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<NUM_COLS, NUM_ROWS, NUM_LAYERS, CustomActions> = [
    /* QWERTY */
    /* 
        All Trans keys are placeholders to even out the layout
        All k(No) keys are functional
    */
    [
    // left side                                                                                                                                              
    [k(Escape),       k(Grave),      k(Kb1),   k(Kb2),  k(Kb3),   k(Kb4),   k(Kb5),   k(F1),    k(F2),     k(F3),       Trans,        Trans,     Trans,     Trans,   Trans,     Trans],
    [Trans,           k(Tab),        k(Q),     k(W),    k(E),     k(R),     k(T),     k(F4),    k(F5),     k(F6),       Trans,        Trans,     Trans,     Trans,   Trans,     Trans],
    [Trans,           k(CapsLock),   k(A),     k(S),    k(D),     k(F),     k(G),     k(F7),    k(F8),     k(F9),       Trans,        Trans,     Trans,     Trans,   Trans,     Trans],
    [k(NonUsBslash),  k(LShift),     k(Z),     k(X),    k(C),     k(V),     k(B),     k(F10),   k(F11),    k(F12),      Trans,        Trans,     Trans,     Trans,   Trans,     Trans],
    [Trans,           k(LCtrl),      k(LGui),  k(LAlt), Trans,    k(LCtrl), k(Space), Trans,    Trans,     Trans,       Trans,        Trans,     Trans,     Trans,   Trans,     Trans,],
    // right side                                                                                                                                                   
    [k(MediaVolDown), k(MediaVolUp), k(Mute),  Trans,   k(Kb6),   k(Kb7),   k(Kb8),   k(Kb9),   k(Kb0),    k(Minus),    k(Equal),     k(BSpace), Trans,     Trans,   Trans,     Trans],
    [k(Intl1),        k(Intl2),      k(Intl3), Trans,   k(Y),     k(U),     k(I),     k(O),     k(P),      k(LBracket), k(RBracket),  k(Enter),  k(Insert), k(Home), k(PgUp),   Trans],
    [k(Intl4),        k(Intl5),      k(Intl6), Trans,   k(H),     k(J),     k(K),     k(L),     k(SColon), k(Quote),    k(NonUsHash), k(Enter),  k(Delete), k(End),  k(PgDown), Trans],
    [k(Intl7),        k(Intl8),      k(Intl9), Trans,   k(N),     k(M),     k(Comma), k(Dot),   k(Slash),  k(RShift),   k(RShift),    Trans,     Trans,     k(Up),   Trans,     Trans],
    [Trans,           Trans,         Trans,    Trans,   k(Space), k(RCtrl), Trans,    Trans,    Trans,     k(Space),    k(RCtrl),     Trans,     k(Left),   k(Down), k(Right),  Trans,],
    ] 
];
