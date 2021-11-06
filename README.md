# ERASE v0.1.0
***Categories:** meta-plugin, filter, distortion*

## Installation
_**Disclaimer:** this plugin will only work on 64-bit windows machines!_ \
Download the `.dll` file in the `bin/` directory and place it into your DAW's VST folder.
Previous versions of the plugin are also available, in case you need them.

## Compiling The Source Code
_**Note:** you don't need to compile the source code if you just want to use the plugin, just download the `.dll`._ \
Make sure you have Cargo installed on your computer (the Rust compiler). Then in the root of the repository run `cargo build`. Once Cargo is done building, there should be a `ERASE_v0_1_0.dll` file in the newly created `debug/` directory. Place this file into your DAW's VST folder.

# What is ERASE?
Remember the old version of HYSTERESIS that had all those features that I mercilessly removed? Well here's one of them as a stand-alone plugin: the erase knob. What is it? It's a slew-limiter where you can choose between softer or harder slew clipping. What does that mean? It's a sort of dirty distorted low-pass filter where things tend to look more like triangle waves the more you filter them.

How to get a similar sound as the erase knob in HYSTERESIS v0.2.0: turn hardness to 0.5, pre/post to 0dB, inv/dry/wet to 0.5 and play with erase knob.

How to emulate the sound of the old HYSTERESIS plugin: chain an instance of HYSTERESIS v0.3.1 or newer and an instance of ERASE.

## Controls Explained
+ erase: amount of slew applied
+ hardness: hardness of the clipping function used for slewing
+ post gain: amplify or attenuate after the effect
+ inv/dry/wet: crossfades between dry - wet, dry, wet. Using it on the inverted mode (dry - wet) is useful for isolating transients.

