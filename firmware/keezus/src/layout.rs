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
[k(Escape),       Trans,         k(Home),   Trans,       k(End),    Trans,    k(End),    Trans,   Trans,    Trans,    Trans,    Trans,    Trans,       Trans,    Trans,       Trans],
[k(RAlt),         Trans,         k(Grave),  k(W),        k(E),      k(R),     k(T),      Trans,   Trans,    k(F1),    k(F2),    k(F3),    Trans,       Trans,    Trans,       Trans],
[k(Tab),          Trans,         k(Q),      k(S),        k(D),      k(F),     k(G),      Trans,   Trans,    k(F4),    k(F5),    k(F6),    Trans,       Trans,    Trans,       Trans],
[k(BSpace),       Trans,         k(A),      k(Z),        k(X),      k(C),     k(V),      Trans,   Trans,    k(F7),    k(F8),    k(F9),    Trans,       Trans,    Trans,       Trans],
[k(LShift),       Trans,         k(LShift), k(LAlt),     k(LGui),   k(LCtrl), k(Space),  k(B),    Trans,    k(F10),   k(F11),   k(F12),   Trans,       Trans,    Trans,       Trans,],
// right side                                                                                                                                                    
[k(MediaVolDown), k(MediaVolUp), k(Mute),   k(CapsLock), k(Insert), Trans,    Trans,     Trans,   k(Left),  Trans,    k(Right), Trans,    k(PgUp),     Trans,    k(PgDown),   Trans],
[k(Kb1),          k(Kb2),        k(Kb3),    k(Enter),    k(Delete), k(Home),  k(End),    Trans,   k(Y),     k(U),     k(I),     k(O),     k(LBracket), Trans,    k(RBracket), Trans],
[k(Kb4),          k(Kb5),        k(Kb6),    k(Enter),    k(PgUp),   k(Up),    k(PgDown), k(Up),   k(H),     k(J),     k(K),     k(L),     k(P),        Trans,    k(Quote),    Trans],
[k(Kb7),          k(Kb8),        k(Kb9),    Trans,       k(Left),   k(Down),  k(Right),  Trans,   k(N),     k(M),     k(Comma), k(Dot),   k(SColon),   Trans,    k(Enter),    Trans],
[k(Minus),        k(Kb0),        k(Equal),  k(RCtrl),    Trans,     Trans,    Trans,     k(Down), k(Space), k(RCtrl), k(Left),  k(Right), k(Slash),    Trans,    k(RShift),   Trans,],
] 
];
