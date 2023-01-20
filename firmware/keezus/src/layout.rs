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
[k(Escape),       k(Grave),      k(Kb1),   k(Kb2),  k(Kb3),   k(Kb4),   k(Kb5),   k(F1),    k(F2),     k(F3),       Trans,        Trans,       Trans,     Trans,   Trans,     Trans],
[Trans,           k(Tab),        k(Q),     k(W),    k(E),     k(R),     k(T),     k(F4),    k(F5),     k(F6),       Trans,        Trans,       Trans,     Trans,   Trans,     Trans],
[Trans,           k(BSpace),     k(A),     k(S),    k(D),     k(F),     k(G),     k(F7),    k(F8),     k(F9),       Trans,        Trans,       Trans,     Trans,   Trans,     Trans],
[k(NonUsBslash),  k(LShift),     k(Z),     k(X),    k(C),     k(V),     k(B),     k(F10),   k(F11),    k(F12),      Trans,        Trans,       Trans,     Trans,   Trans,     Trans],
[Trans,           k(LCtrl),      k(LGui),  Trans,   k(LAlt),  k(LCtrl), k(Space), Trans,    Trans,     Trans,       Trans,        Trans,       Trans,     Trans,   Trans,     Trans,],
// right side                                                                                                                                                   
[k(MediaVolDown), k(MediaVolUp), k(Mute),  Trans,   k(Kb6),   k(Kb7),   k(Kb8),   k(Kb9),   k(Kb0),    k(Minus),    k(Equal),     k(CapsLock), k(Insert), Trans,   Trans,     Trans],
[k(Kp1),          k(Kp2),        k(Kp3),   Trans,   k(Y),     k(U),     k(I),     k(O),     k(P),      k(LBracket), k(RBracket),  k(Enter),    k(Delete), k(Home), k(End),    Trans],
[k(Kp4),          k(Kp5),        k(Kp6),   Trans,   k(H),     k(J),     k(K),     k(L),     k(SColon), k(Quote),    k(NonUsHash), k(Enter),    k(PgUp),   k(Up),   k(PgDown), Trans],
[k(Kp7),          k(Kp8),        k(Kp9),   k(Kp0),  k(N),     k(M),     k(Comma), k(Dot),   k(Slash),  k(RShift),   k(RShift),    Trans,       k(Left),   k(Down), k(Right),  Trans],
[Trans,           Trans,         Trans,    Trans,   k(Space), k(RCtrl), Trans,    Trans,    Trans,     k(Space),    Trans,        k(RCtrl),    Trans,     Trans,   Trans,     Trans,],
] 
];
