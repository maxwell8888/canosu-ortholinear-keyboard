# Canosu keyboard

## Design for from scratch software

No ctrl, window, or alt key necessary, when not in a text editing mode all actions/shortcuts are performed with alphanumeric keys directly (will still feel like using modifiers though because we will be making use of shift and fn). Then in text mode we will have conventional style text editing and not force vim style modaling, the only thing we need to do is reserve some of the fn keys for text nav, i.e.:

-   up/down/left/right
-   left/right word (ctrl left/right)
-   home/end line
-   home/end page (ctrl home/end)
-   look around (ctrl up/down)
-   all combinations of bspace/delete char/word
-   cut/copy/paste
-   undo/redo
-   spell check
-   select all, save, format, bold?

which is a lot, and would require an additional layer which ruins our simple two thumb design and would introduce needing to e.g. hold two keys with one thumb.
We could reduce these requirements a lot by leaning into a "visual mode" where shift isn't used in combination which these nav keys and instead the use just has to activate the visual mode with a given key (could just do this all on the keyboard with a layer key?) and then that we could redefine e.g. shift+left to mean cursor move one word left rather than highlight 1 char left.
We could also reduce the bspace/delete combinations to a single thing and rely on doing first making selections in visual mode before deleting, which works in theory but means bspace turns from being a single keypress to 3 - activate visual mode, select char to left, delete.
One problem (when considering "legacy" apps) is that most apps use ctrl+shift+pgup/down to reposition tabs (ie vscode and chrome, both of which I think I could change the shortcuts for (also navigating tabs has nothing to do which pageup/pagedown and I think it would be better to use something else like U/O anyway)).
Lets consider the most common use cases:

-   Highlight the word to the left
    Currently this is very quick and comfortable which ctrl+shift+left. We are proposing hold fn, tap visual which could be e.g. V, shift+h which is definitely more complex but pretty reasonable. Could actually just have a different layer for visual mode so it would be fn2+shift(which behaves as ctrl)+h so exactly the same. So the drawback is that we are introducing a second fn layer but it will never be used simultaneously with the main fn layer so will never cause any double thumbing problems.

Something like this could work well and even be compatible with legacy apps/OS. We are just replacing shift with another layer so we can choose how we want it to behave for different keys.

For commands like cut paste etc, these would just be the normal keys used but you need to enter visual mode.

Maybe a better approach is to simply have a second layer on the same thumb side as fn which won't matter because they would never need to be pressed simultaneously. Could even basically just be a Ctrl layer (for compatibility of using Ctrl shortcuts on legacy) but with the spare keys assigned to the ctrl+nav commands (though there are around 6 of these and basically no space around the alpha keys especially given we would also want to be able to do e.g. ctrl+. etc).
Also with a visual mode we shouldn't need to use fn when navigating with hjkl etc because we never need to input text in visual mode, only navigate. What about if we want to start entering text to replace the current selection?? This would mean an extra keypress? Could just use fn as a single modifier hold is not bad and that will be the expected way to have to navigate while text editing anyway.

## Design

Dedicated arrow keys vs layer
This is the fundamental question most central to a good user experience. We ofcourse want to reserve the home row for alpha keys and comfortable text input, but we also want to be able to scroll/navigate without a modifier for comfort because it is probably the single most used use case. We can have software like vimium and helix that facilitates this but there will always be a tension here, eg we want to be able to type text in a search box but also hit down to make selections - surely we can just use tab in this case? Though J would be more natural since that is what we are used to for "down".

Is it really only stuff like copy/paste that we want to do while text editing? What about opening the jump list, changing the tab, etc. Do we really want to require someone "exits" somehow before these commands? Well considering the commands can then be a single key, it is no more key presses. I think really we want to start thinking of a key which is dual use for escape/ctrl ie exiting text edit mode into nav/command mode but also just running commands adhoc whilst in text edit. I don't think it would work using the current escape key position but we could use the right hand ctrl ie make it so a single tap is escape / nav/command mode and hold+key just works like a normal ctrl.

Re wanting to be able to do things/commands whilst editing text but also not wanting to have to rely heavily on Ctrl, I've previously mentioned it is a good idea to consider use of shift separately ie capitalising (more like a layer) vs highlighting (more like a mode). Similarly it makes sense to separate uses of ctrl whilst text editing. Ctrl+nav like move word could/should be independant layer keys (ie [RCtrl Left]) so we are not relying on ctrl. For commands like paste, they are not (typically) repeated commands (like writing CAPITALS or numbers 45624) so they don't need or thumb key or easy to reach key, they could be trigger with taps eg tap the very bottom ring finger key then V or whatever. Using taps is a great way to avoid double thumbing.
Even without using taps, we should always be clear about what actions are repeated (like writing CAPITALS or numbers 45624) and which actions are one offs and so don't necessarily need to make use of left and right thumbs.

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
