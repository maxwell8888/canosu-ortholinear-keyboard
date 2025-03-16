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
// NOTE it is annoying not having Ctrl on might right hand for web browsing/mouse clicking so for now have swapped visual mode and ctrl, but eventually visual mode should be on the same side the normal fn layer. Also maybe don't think of it as visual mode ie highlighting only, but everything you might want to do while editing text. eg paste is one of the things but you don't need to highlight things to paste. Have also added a pinky stretch LCtrl which can hopefully replace the right side ctrl gradually allowing it to be replace with the second layer. asdfa
{
// left side
[n       n      n                 n    n    n      n                   n n n n n n n n n]
[n       n      W                 E    R    T      {DefaultLayer(10)}  n n n n n n n n n]
[Tab     Q      S                 D    F    G      MediaScrollUp       n n n n n n n n n]
[BSpace  A      X                 C    V    B      MediaScrollDown     n n n n n n n n n]
[RCtrl   Z      LAlt              (11) n    LShift LGui                n n n n n n n n n]
// right side
[n n n n n n n n n             n        {DefaultLayer(3)}     n     n      n        n       n]
[n n n n n n n n MediaMute     Y        U                     I     O      PgUp     PgDown  n]
[n n n n n n n n MediaVolUp    H        J                     K     L      P        Enter   n]
[n n n n n n n n MediaVolDown  N        M                     Up    Escape Space    Home    n]
[n n n n n n n n RCtrl         (1)      Left                  Down  Right  n        End     n]
}
{
// layer 1 - fn layer
// TODO update keyberon so I can replace SColon with ':'
// NOTE @ is " and " is @
// With the current fn key position . and , are actually more comfortable and the right hand side next to the numbers which is also useful for one handed decimal numbers. But if the fn key was in the shift position or on the right hand then the left hand homing positions are of course better than having to move to the H position with the right hand. Maybe try getting used to using the fn key in the outer modifier position on the right hand? The problem with this is that using numbers and backslash with the right hand fn key is uncomfortable... but also not tooo bad?
// Would be nice to have ! slightly more accessible, as it is probably the next most commonly used punctuation after ?
// frequency of use outside of coding: , . ? ! () @ £ $ / : '' "" + - =

// Still to fix/find solution for:
// Ctrl+symbol/number, in particular:
// Ctrl+/ for comment lines
// Ctrl =/- for zooming
// Delete word
// I think making Ctrl tapable word work for all of these

// left side
[n         n            t    t    t  t      n n n n n n n n n n]
[n         Grave        1    2    3  =      n n n n n n n n n n]
[n         n            4    5    6  0      n n n n n n n n n n]
[Delete    n            7    8    9  -      n n n n n n n n n n]
[n         NonUsHash    t    t    t  t      n n n n n n n n n n]
// right side
// [n n n n n n n n n   t     n            n     n             t        t            t]
// [n n n n n n n n n   '`'   [RCtrl Left] Up    [RCtrl Right] '['      ']'          t]
// [n n n n n n n n n   @     Left         Down  Right         '{'      '}'          t]
// [n n n n n n n n n   '"'   n            n     n             SColon   Quote        t]
// [n n n n n n n n n   RCtrl t            n     n             /        NonUsBslash  t]

// I'm not a fan of having Ctrl+navkey in a different position to navkey. Yes we can treat them as separate things but also the traditional approach of treating things like a "matrix" where we can have any combo of direction/ctrl/shift is powerful and intuitive, and trying to flatten that out makes things larger not smaller and more concise. I think a second fn layer which is identical but everything is sent with ctrl would be preferrable. The big down side is that we will still need a normal ctrl key for non-fn commands, so now we have two extra keys - ctrl and fn2/ctrl-fn.
// Maybe the best solution is actually to have ctrl on the left, use tapping to avoid need to double thumb ctrl and shift, or just accept it is not that commonly used for shortcuts, and stick with using a different position for ctrl+navkey to navkey.
// Not a huge fan of having . which is a very important key (especially for programming) a) not on the home row b) using a modifier on the same hand which combined make it far less comfortable to use. A solution might be to just put it in the primary layer, maybe in escapes position (it's normal position) and move escape to the left or to ctrl as a tap key.
// Also given by default foo. will bring up intellisene (though I potentially prefer it to be triggered manually), it seems like ctrl . (or similar) is a good key combination for bringing up autocomplete/menus(?).
// Should all nav be on the left hand (consistent for gamers) and just upset the vimmers but at least number input will be more natural, on the same hand as fn for one handed input, and allows , and . to be on a separate hand to the modifier making them slightly more comfortable (though still not on the home row and we also really want to be using the most comfortable positions for nav word left/right anyway...).
// We probably actually just want to switch the fn and shift keys...
[n n n n n n n n n   t             t             t             t                 t        t            n]
[n n n n n n n n n   [LCtrl Up]    [LCtrl Left]  [LCtrl Right] [LCtrl Down]      n        n            n]
[n n n n n n n n n   Left          Down          Up            Right             '['      ']'          n]
[n n n n n n n n n   n             n             ,             .                 SColon   Quote        n]
[n n n n n n n n n   t             t             n             n                 /        NonUsBslash  n]
}
{
// layer 2 - manual shift layer
// We want to decouple shift from capitalising letters and highlighting letters, so that we can instead use is to mean ctrl for arrows etc.
// left side
[n         n                t           t           t           t               n n n n n n n n n n]
[n         [LCtrl Delete]   [LShift 1]  [LShift 2]  [LShift 3]  [LShift =]      n n n n n n n n n n]
[n         [LShift Tab]     [LShift 4]  [LShift 5]  [LShift 6]  [LShift 0]      n n n n n n n n n n]
[n         [LCtrl BSpace]   [LShift 7]  [LShift 8]  [LShift 9]  [LShift -]      n n n n n n n n n n]
[n         NonUsHash        t           t           t           t               n n n n n n n n n n]
// right sides
[n n n n n n n n n   t               t             t              t                t                 t                     n]
[n n n n n n n n n   [RShift Grave]  [RCtrl Home]  [RCtrl End]    n                [RShift Enter]    n                     n]
[n n n n n n n n n   [RCtrl Left]    [RCtrl Down]  [RCtrl Up]     [RCtrl Right]    [RShift '[']      [RShift ']']          n]
[n n n n n n n n n   [RShift Quote]  n             [RShift ,]     [RShift .]       [RShift SColon]   n                     n]
[n n n n n n n n n   t               t             n              n                [RShift /]        [RShift NonUsBslash]  n]
}

{
// layer 3 - VISUAL MODE
// The whole point of this is pretending we are using a from scratch design where we don't use ctrl so to do things like copy, paste, move by word, etc we need a "visual mode". Not totally sure how this is different from just using ctrl? Because it combines holding ctrl *and* fn. There is duplicates for commands used during text editing because the intention is to mimick how it would work if we had a from scratch setup with no ctrl key whatsoever.
// left side
[n       n           n                 n            n             n              n                  n n n n n n n n n]
[n       n           W                 E            R             T              {DefaultLayer(10)} n n n n n n n n n]
[n       n           S                 D            F             G              MediaScrollUp      n n n n n n n n n]
[n       [LCtrl A]   [LCtrl X]         [LCtrl C]    [LCtrl V]     [LCtrl B]      MediaScrollDown    n n n n n n n n n]
[n       [LCtrl Z]   n                 n            n             LCtrl          LGui               n n n n n n n n n]
// right side
[n n n n n n n n n   t               t              t               t                 t        t            n]
[n n n n n n n n n   n               [RShift Home]  [RShift End]    n                 n        n            n]
[n n n n n n n n n   [RShift Left]   [RShift Down]  [RShift Up]     [RShift Right]    n        n            n]
[n n n n n n n n n   n               n              n               n                 n        n            n]
[n n n n n n n n n   n               n              n               n                 n        n            n]
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
[t           RAlt       n      W    E         R     T      n n n n n n n n n]
[t           n            Q      S    D         F     G      n n n n n n n n n]
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
[n n n n n n n n n   t    -    0    =    n        NonUsBslash  t]  
}
{
// layer 8
// left side                                                                                                                                              
[t           Escape     Home   t   End        t     End    n n n n n n n n n]
[t           RAlt       n      W    E         R     T      n n n n n n n n n]
[t           n          Q      S    D         F     G      n n n n n n n n n]
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

// layer 10 Colemak dh
{
// left side                                                                                                                                              
[n       n      n                 n n    n      n                 n n n n n n n n n]
[Escape  n      W                 E R    T      {DefaultLayer(0)} n n n n n n n n n]
[n       Q      S                 D F    G      MediaScrollUp     n n n n n n n n n]
[BSpace  A      X                 C V    B      MediaScrollDown   n n n n n n n n n]
[n       Z      LAlt              n (1)  LShift LGui              n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n             n        {DefaultLayer(3)}     n     n      n        n       n]
[n n n n n n n n MediaMute     Y        U                     I     O      PgUp     PgDown  n]
[n n n n n n n n MediaVolUp    H        J                     K     L      P        Enter   n]
[n n n n n n n n MediaVolDown  N        M                     Space n      ;        Home    n]
[n n n n n n n n (1)           RCtrl    Left                  Down  Right  n        End     n]
}

// layer 11 Numbers
{
// left side                                                                                                                                              
[n       n      n                 n n    n      n                 n n n n n n n n n]
[Escape  n      W                 E R    T      {DefaultLayer(0)} n n n n n n n n n]
[n       Q      S                 D F    G      MediaScrollUp     n n n n n n n n n]
[BSpace  A      X                 C V    B      MediaScrollDown   n n n n n n n n n]
[n       Z      LAlt              t (1)  LShift LGui              n n n n n n n n n]
// right side                                                                                                                                                    
[n n n n n n n n n    n        t    t    t   n  n n]
[n n n n n n n n n    n        1    2    3   n  n n]
[n n n n n n n n n    n        4    5    6   n  n n]
[n n n n n n n n n    n        7    8    9   n  n n]
[n n n n n n n n n    RCtrl    -    0    +   n  n n]
}

};
