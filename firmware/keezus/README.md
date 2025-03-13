# Canosu keyboard

## Design

We want typing to feel more <insert gif of cat smashing alternate hands on keyboard> and less <gif of some kind of awkard position like a really stretch chord on a guitar or a game of twister>.

The major problem with my idea of having separate keys for e.g. 1 and ! is that we still need shift for letters, but shift for ! is meaningless so we have a bunch of useless/doesn't make sense modifier combinations. To make it make sense, would need to send a new code for ! and then have a custom keyboard layout installed on the OS, and also no applications in the world would have support for shift ! so it would only be useful in applications I write - actually if I do my plan of having per application remappings/making shortcuts consistent across apps using something like xeremaps then can just map shift ! to something useful at that point.
Though what would shift-! do in insert mode? still doesn't really make sense. Could make it do other stuff like arrow keys or escape or something, but then why not just use 1 -shift-> ! and use other keys for arrows or escape or whatever?
Using 1 -shift-> ! means we would need less keys overall which is useful for mapping to a games controller. This is in tension with wanting to avoid chords. Fundamentally we will avoid chords by using modal mode, and may keys like home like become redundant (or maybe still used in insert mode?) so we don't even really need to assign the to the leftover fn keys... well what _do_ we use the leftover fn keys for, there is 10 of them? Avoid using the awkward ones like tgbyhn for symbols and instead use them for other things.. volume? we want volume to be modifier free...
a 20key board could be called the 20/20 (vision) which is a pretty cool name
Arguably shift is stupid and shouldn't even exist anyway and should just use caps lock? I don't think using capslock really changes much, would still want it to be able to do holds and/or taps which wouldn't make it much different to shift...
In order to incorportate legacy software whilst still providing a new clean light experience, can lean on the alt-f type of shortcuts that are common in desktop apps so instead of alt-f -> r you just do f -> r because you are in modal mode, and also try and alternate hands when choosing keys. One major problem is that most applications have the shortcuts printed in the app. Just have to deal with this and expect users to know they are redundant? Well I think the alt dropdown menus only use characters anyway so can actually just use the same shortcuts. I think it would be tricky to distinguish between alt-f and f since we would be wanting to use the same key from the use but map to alt-f or f depending only on whether a dropdown is open. Could technically track this but seems hacky and error prone.

To really design a keyboard layout properly we want a piece of software that not only tracks individual keys, but combinations, for example how often the the combination ctrl+shift+home used? How often do we actually use enter and is it repeated (and thus shouldn't need a modifier) or quite one off (and therefore being a fn key wouldn't matter). I kind of like the idea of delete or backspace or enter being fn keys since they are the most important keys, eg send email, permanently delete this thing, etc (typically this is actually all just enter, delete/backspace actions are usually recoverable), so (arguably?) less chance of accidentally fumbling a keydown and doing some irreversible, unintentional action.

## Solutions for replacing ctrl key

1. Double tap fn. Tap fn once and then again within 1 second and hold and it becomes ctrl. I can't see how this could be mixed up for anything. Ofcourse this isn't useful for actually replacing ctrl since it has many limitations like you can't do ctrl+1 for example. It is useful however for allowing easy access to another modifier/layer without needing to use up a key, which is useful e.g. for triggering a shortcut when in text edit mode. Probably something I would make use of if designing my own system from scratch.
1. Chords eg fn+j+k, the problem is that this can't distinguish from someone simply type 45 fast. The solution

## Dependencies

rustup default beta
cargo install flip-link
cargo install probe-run
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs

## Flash Code

Hold the "USB Boot" button (near the QSPI chip), and either press the reset button or re-insert the USB cable to put the board in USB mass-storage bootloader mode.

cargo run --release

## Troubleshooting

If you get an error such as:

Error: "Memory segment 0x010000->0x010094 is outside of valid address range for device"
Double check that your RUSTFLAGS environment variable, as it will take precedence over the values set in ./cargo/config.toml.
