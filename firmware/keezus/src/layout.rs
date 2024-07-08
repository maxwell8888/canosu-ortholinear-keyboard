use keyberon::action::{k, l, m, Action, Action::*};
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
pub static LAYERS: keyberon::layout::Layers<NUM_COLS, NUM_ROWS, NUM_LAYERS, CustomActions> = keyberon::layout::layout! {
// Linux
// Goals
// Type numbers without needing to down thumb on same hand, but also want to be able to do numbers one handed... could just use a locking layer change eg double tap for 1 handed numbers
// Having . on the same hand as numbers for 1 handed typing seems nice but tab would probably be more useful... 
{
// left side                                                                                                                                              
[n       n      n                 n n    n      n               n n n n n n n n n]
[Escape  Grave  W                 E R    T      t               n n n n n n n n n]
[Tab     Q      S                 D F    G      MediaScrollUp   n n n n n n n n n]
[BSpace  A      X                 C V    B      MediaScrollDown n n n n n n n n n]
[n       Z      LAlt              n (1)  LShift LGui            n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n             n        {DefaultLayer(3)}     n     n      n        n       n]
[n n n n n n n n MediaMute     Y        U                     I     O      PgUp     PgDown  n]
[n n n n n n n n MediaVolUp    H        J                     K     L      P        Enter   n]
[n n n n n n n n MediaVolDown  N        M                     Up    n      Space    Home    n]
[n n n n n n n n (1)           RCtrl    Left                  Down  Right  n        End     n]
}
{
// layer 1
// TODO update keyberon so I can replace SColon with ':'
// NOTE @ is " and " is @
// With the current fn key position . and , are actually more comfortable and the right hand side next to the numbers which is also useful for one handed decimal numbers. But if the fn key was in the shift position or on the right hand then the left hand homing positions are of course better than having to move to the H position with the right hand. Maybe try getting used to using the fn key in the outer modifier position on the right hand? The problem with this is that using numbers and backslash with the right hand fn key is uncomfortable... but also not tooo bad?
// Would be nice to have ! slightly more accessible, as it is probably the next most commonly used punctuation after ?
// frequency of use outside of coding: , . ? ! () @ £ $ / : '' "" + - =
// left side                                                                                                                                              
[n         n            t     t      t      t      n n n n n n n n n n]
[n         NonUsHash    !     '('    ')'    ^      n n n n n n n n n n]
[|         *            .     ,      &      SColon n n n n n n n n n n]
[Delete    ?            =     <      >      $      n n n n n n n n n n]
[%         '_'          n     n      n      n      n n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n   t     t    t    t    t        t            t]
[n n n n n n n n n   '`'   1    2    3    '['      ']'          t]
[n n n n n n n n n   @     4    5    6    '{'      '}'          t]
[n n n n n n n n n   '"'   7    8    9    SColon   Quote        t]
[n n n n n n n n n   RCtrl -    0    +    /        NonUsBslash  t]
}
{
// layer 2
// left side                                                                                                                                              
[t           Escape     Home   t   End        t     End    n n n n n n n n n]
[t           RAlt       Grave  W    E         R     T      n n n n n n n n n]
[t           t          Q      S    D         F     G      n n n n n n n n n]
[t           Delete     A      Z    X         C     V      n n n n n n n n n]
[t           LShift     LShift LAlt LGui      LCtrl Space  n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n n     n     n      n   n   n   n]
[n n n n n n n n n n     F1    F2     F3  n   L   n]
[n n n n n n n n n n     F4    F5     F6  n   n   n]
[n n n n n n n n n n     F7    F8     F9  n   n   n]
[n n n n n n n n n t     F10   F11    F12 n   n   n]
}




// MacOS Normal
{
// layer 3
// left side                                                                                                                                              
[n       n      n     n     n    n      n               n n n n n n n n n]
[Escape  Grave  W     E     R    T      t               n n n n n n n n n]
[Tab     Q      S     D     F    G      MediaScrollUp   n n n n n n n n n]
[BSpace  A      X     C     V    B      MediaScrollDown n n n n n n n n n]
[n       Z      LAlt  LCtrl (4)  RShift LGui            n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n             n     {DefaultLayer(6)} t     t      t        t       t]
[n n n n n n n n MediaMute     Y     U                 I     O      PgUp     PgDown  t]
[n n n n n n n n MediaVolUp    H     J                 K     L      P        Enter   t]
[n n n n n n n n MediaVolDown  N     M                 Up    Up     Space    Home    t]
[n n n n n n n n (4)           RGui  Left              Down  Right  Down     End     t]
}
{
// layer 4
// left side                                                                                                                                              
[n         n            t     t      t      t      n n n n n n n n n n]
[Escape    NonUsHash    !     @      $      ^      n n n n n n n n n n]
[<         >            .     ,      (5)    SColon n n n n n n n n n n]
[Delete    ?            '('   ')'    &      %      n n n n n n n n n n]
[*         '"'          t     |      t      t      n n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n   t    t    t    t    t        t            t]
[n n n n n n n n n   ~    1    2    3    '['      ']'          t]
[n n n n n n n n n   .    4    5    6    '{'      '}'          t]
[n n n n n n n n n   ,    7    8    9    SColon   Quote        t]
[n n n n n n n n n   t    -    0    =    /        NonUsBslash  t]
}
{
// layer 5
// left side                                                                                                                                              
[t           Escape     Home   t   End        t     End    n n n n n n n n n]
[t           RAlt       Grave  W    E         R     T      n n n n n n n n n]
[t           t          Q      S    D         F     G      n n n n n n n n n]
[t           Delete     A      Z    X         C     V      n n n n n n n n n]
[t           LShift     LShift LAlt LGui      LCtrl Space  n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n n     n     n      n   n   n   n]
[n n n n n n n n n n     F1    F2     F3  n   M   n]
[n n n n n n n n n n     F4    F5     F6  n   n   n]
[n n n n n n n n n n     F7    F8     F9  n   n   n]
[n n n n n n n n n t     F10   F11    F12 n   n   n]
}




// MacOS Linux Like
{
// layer 6
// left side                                                                                                                                              
[n       n      n     n     n    n      n               n n n n n n n n n]
[Escape  /      W     E     R    T      t               n n n n n n n n n]
[Tab     Q      S     D     F    G      MediaScrollUp   n n n n n n n n n]
[BSpace  L      X     C     V    B      MediaScrollDown n n n n n n n n n]
[n       Z      LAlt  LCtrl (4)  RShift LGui            n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n             n     {DefaultLayer(0)} t     t      t        t       t]
[n n n n n n n n MediaMute     Y     U                 I     O      PgUp     PgDown  t]
[n n n n n n n n MediaVolUp    H     J                 K     L      P        Enter   t]
[n n n n n n n n MediaVolDown  N     M                 Up    Up     Space    Home    t]
[n n n n n n n n (4)           (9)   Left              Down  Right  Down     End     t]
}
{
// layer 7
// left side                                                                                                                                              
[n         n            t     t      t      t      n n n n n n n n n n]
[Escape    NonUsHash    !     @      $      ^      n n n n n n n n n n]
[<         >            .     ,      (5)    SColon n n n n n n n n n n]
[Delete    ?            '('   ')'    &      %      n n n n n n n n n n]
[*         '"'          t     |      t      t      n n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n   t    t    t    t    t        t            t]
[n n n n n n n n n   ~    1    2    3    '['      ']'          t]
[n n n n n n n n n   .    4    5    6    '{'      '}'          t]
[n n n n n n n n n   ,    7    8    9    SColon   Quote        t]
[n n n n n n n n n   t    -    0    =    Grave    NonUsBslash  t]
}
{
// layer 8
// left side                                                                                                                                              
[t           Escape     Home   t   End        t     End    n n n n n n n n n]
[t           RAlt       Grave  W    E         R     T      n n n n n n n n n]
[t           t          Q      S    D         F     G      n n n n n n n n n]
[t           Delete     A      Z    X         C     V      n n n n n n n n n]
[t           LShift     LShift LAlt LGui      LCtrl Space  n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n n     n     n      n   n   n   n]
[n n n n n n n n n n     F1    F2     F3  n   M   n]
[n n n n n n n n n n     F4    F5     F6  n   n   n]
[n n n n n n n n n n     F7    F8     F9  n   n   n]
[n n n n n n n n n t     F10   F11    F12 n   n   n]
}

{
// layer 9 Right Cmd/Ctrl
// left side                                                                                                                                              
[n       n      n     n         n        n        n               n n n n n n n n n]
[Escape  /      W     E         R        [LGui T] t        n n n n n n n n n]
[Tab     Q      S     D         F        G        MediaScrollUp   n n n n n n n n n]
[BSpace  A      X     [LGui C]  [LGui V] B        MediaScrollDown n n n n n n n n n]
[n       Z      LAlt  LCtrl     (4)      RShift   LGui            n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n             n     {DefaultLayer(0)} t     t      t             t              t]
[n n n n n n n n MediaMute     Y     U                 I     O      [RCtrl PgUp]  [RCtrl PgDown] t]
[n n n n n n n n MediaVolUp    H     J                 K     L      P             Enter          t]
[n n n n n n n n MediaVolDown  N     M                 Up    Up     Space         Home           t]
[n n n n n n n n (4)           (6)   Left              Down  Right  Down          End            t]
}
    

// {
// // layer 6 - Cmd
// // left side                                                                                                                                              
// [n         n     n           n      CapsLock n        n               n n n n n n n n n]
// [Escape    Grave [LGui W]    E      [LGui R] [LGui T] t               n n n n n n n n n]
// [Tab       Q     [LGui S]    D        [LGui F] G      MediaScrollUp   n n n n n n n n n]
// [BSpace    [LGui A] [LGui X] [LGui C] [LGui V] B      MediaScrollDown n n n n n n n n n]
// [{DefaultLayer(0)} [LGui Z] LAlt  LCtrl (4)    RShift (5)             n n n n n n n n n]
// // right side                                                                                                                                                    
// [n n n n n n n n t          t     t     t     t      t        t       t]
// [n n n n n n n n MediaMute  [RGui Y] [RGui U] [RGui I] O [RGui LAlt Left]     [RGui LAlt Right]  t]
// [n n n n n n n n MediaVolUp    [RGui H]     [RGui J]     K     [RGui L] [RGui P]  Enter   t]
// [n n n n n n n n MediaVolDown  N     M     [RGui Up]    Up     Space    Home    t]
// [n n n n n n n n (4)           RGui  [RGui Left]  [RGui Down]  [RGui Right]  Down     End     t]
// }

};

// [
// /* QWERTY */
// /*
//     All Trans keys are placeholders to even out the layout
//     All k(No) keys are functional
// */
// [
// // base layer
// // left side
// [k(Escape),       Trans,         k(Escape), Trans, k(CapsLock), k(End),    Trans,              Trans, Trans, Trans,  Trans,  Trans,  Trans, Trans, Trans, Trans],
// [k(Escape),       k(Grave),      k(W),      k(E),  k(R),        k(T),      Trans,              Trans, Trans, k(F1),  k(F2),  k(F3),  Trans, Trans, Trans, Trans],
// [k(Tab),          k(Q),          k(S),      k(D),  k(F),        k(G),      k(MediaScrollUp),   Trans, Trans, k(F4),  k(F5),  k(F6),  Trans, Trans, Trans, Trans],
// [k(BSpace),       k(A),          k(X),      k(C),  k(V),        k(B),      k(MediaScrollDown), Trans, Trans, k(F7),  k(F8),  k(F9),  Trans, Trans, Trans, Trans],
// [k(RAlt),         k(Z),          k(LAlt),   l(1),  l(2),        k(RShift), k(LGui),            Trans, Trans, k(F10), k(F11), k(F12), Trans, Trans, Trans, Trans,],
// // right side
// [k(MediaVolDown), k(MediaVolUp), k(Mute),   Trans, k(CapsLock), k(Insert), Trans,    Trans,     Trans,           Trans,    k(PgUp),  Trans,    k(PgDown), k(PgUp),     k(PgDown),  Trans],
// [k(Kb1),          k(Kb2),        k(Kb3),    Trans, k(Enter),    k(Delete), k(Home),  k(End),    k(MediaMute),    k(Y),     k(U),     k(I),     k(O),      k(PgUp),     k(PgDown),  Trans],
// [k(Kb4),          k(Kb5),        k(Kb6),    Trans, k(Enter),    k(PgUp),   k(Up),    k(PgDown), k(MediaVolUp),   k(H),     k(J),     k(K),     k(L),      k(P),        k(Enter),   Trans],
// [k(Kb7),          k(Kb8),        k(Kb9),    Trans, Trans,       k(Left),   k(Down),  k(Right),  k(MediaVolDown), k(N),     k(M),     k(Up),    Trans,     k(Space),    k(Home),   Trans],
// [k(Minus),        k(Kb0),        k(Equal),  Trans, k(RCtrl),    Trans,     Trans,    Trans,     l(2),            k(RCtrl), k(Left),  k(Down),  k(Right),  Trans,       k(End),    Trans,],
// ],

// [
// // layer 1
// // left side
// [Trans,           k(Escape),     k(Home),   Trans,   k(End),       Trans,    k(End),    Trans,   Trans,    Trans,    Trans,    Trans,     Trans,     Trans,       Trans, Trans],
// [Trans,           k(RAlt),       k(Grave),  k(W),    k(E),         k(R),     k(T),      Trans,   Trans,    k(F1),    k(F2),    k(F3),     Trans,     Trans,       Trans, Trans],
// [Trans,           Trans,         k(Q),      k(S),    k(D),         k(F),     k(G),      Trans,   Trans,    k(F4),    k(F5),    k(F6),     Trans,     Trans,       Trans, Trans],
// [Trans,           k(Delete),     k(A),      k(Z),    k(X),         k(C),     k(V),      Trans,   Trans,    k(F7),    k(F8),    k(F9),     Trans,     Trans,       Trans, Trans],
// [Trans,           k(LShift),     k(LShift), k(LAlt), k(LGui),      k(LCtrl), k(Space),  k(B),    Trans,    k(F10),   k(F11),   k(F12),    Trans,     Trans,       Trans, Trans,],
// // right side
// [k(MediaVolDown), k(MediaVolUp), k(Mute),   Trans,   k(CapsLock), k(Insert), Trans,    Trans,     Trans,   k(PgUp),  Trans,    k(PgDown), Trans,  k(PgUp),   k(PgDown),   Trans],
// [k(Kb1),          k(Kb2),        k(Kb3),    Trans,   k(Enter),    k(Delete), k(Home),  k(End),    Trans,   k(Y),     k(F1),    k(F2),     k(F3),  k(LBracket),  k(RBracket), Trans],
// [k(Kb4),          k(Kb5),        k(Kb6),    Trans,   k(Enter),    k(PgUp),   k(Up),    k(PgDown), k(Up),   k(H),     k(F4),    k(F5),     k(F6),  k(Home),   Trans,       Trans],
// [k(Kb7),          k(Kb8),        k(Kb9),    Trans,   Trans,       k(Left),   k(Down),  k(Right),  Trans,   k(N),     k(F7),    k(F8),     k(F9),  k(End),   k(Enter),    Trans],
// [k(Minus),        k(Kb0),        k(Equal),  Trans,   k(RCtrl),    Trans,     Trans,    Trans,     k(Down), k(RCtrl), k(F10),   k(F11),    k(F12), k(Right),   k(RShift),   Trans,],
// ],

// [
// // layer 2
// // left side
// [Trans,         k(Home),         Trans,    k(End),    Trans,    k(End),    Trans,   Trans,    Trans,    Trans,    Trans,    Trans,   Trans,    Trans,       Trans, Trans],
// [Trans,         k(Grave),        keyberon::action::Action::MultipleKeyCodes([LShift, Kb1]),    k(F2),    k(F3),     k(T),      Trans,   Trans,    k(F1),    k(F2),    k(F3),    Trans,   Trans,    Trans,       Trans, Trans],
// [Trans,         k(Q),            k(F4),    k(F5),    l(1),     k(G),      Trans,   Trans,    k(F4),    k(F5),    k(F6),    Trans,   Trans,    Trans,       Trans, Trans],
// [k(Delete),     k(A),            k(F7),    k(F8),    k(F9),     k(V),      Trans,   Trans,    k(F7),    k(F8),    k(F9),    Trans,   Trans,    Trans,       Trans, Trans],
// [Trans,         k(RAlt),         k(F10),   k(F11),   k(F12),    Trans,     Trans,    Trans,   k(F10),   k(F11),   k(F12),   Trans,  Trans,     Trans,       Trans, Trans,],
// // right side
// [k(MediaVolDown), k(MediaVolUp), k(Mute),  Trans, k(CapsLock), k(Insert), Trans,    Trans,     Trans,   Trans,    k(Home),  Trans,    k(End),   k(Home),     k(End),      Trans],
// [k(Kb1),          k(Kb2),        k(Kb3),   Trans, k(Enter),    k(Delete), k(Home),  k(End),    Trans,   k(Y),     k(Kb1),   k(Kb2),   k(Kb3),   k(Home),     k(End),      Trans],
// [k(Kb4),          k(Kb5),        k(Kb6),   Trans, k(Enter),    k(PgUp),   k(Up),    k(PgDown), k(Up),   k(Dot),   k(Kb4),   k(Kb5),   k(Kb6),   k(LBracket), k(RBracket), Trans],
// [k(Kb7),          k(Kb8),        k(Kb9),   Trans, Trans,       k(Left),   k(Down),  k(Right),  Trans,   k(Comma), k(Kb7),   k(Kb8),   k(Kb9),   k(SColon),   k(Quote),    Trans],
// [k(Minus),        k(Kb0),        k(Equal), Trans, k(RCtrl),    Trans,     Trans,    Trans,     Trans,   Trans,    k(Minus), k(Kb0),   k(Equal), k(Slash),    k(Bslash),   Trans,],
// ]
// ];
