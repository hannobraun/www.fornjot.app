+++
title = "Weekly Release - Ready to Fix"
date = 2022-10-24

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.21.0"
subtitle = "Wrapping up the recent infrastructure work, moving on to a bug fix."
+++

I've been continuing my new strategy of working on both the big-picture priority (the union operation, [#42]) and smaller tasks with more immediate pay-off, in parallel. I'm happy with the results so far!

I finished [#1021] and have started to work on [#1162], which is a bug that blocks further progress on the union operation. There's hope that I can fix this bug relatively quickly, but I'm going to write new validation code to do that, and that new validation code might uncover previously unknown issues. So we'll see how all that goes.

On the "small tasks" side of things, I've made some quality of life improvements in the app. Check below for details!

And in addition, [@erenoku] has contributed a small but important improvement: A timestamp is now shown for each status message!


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Improve error message, if model can't be loaded ([#1235])
- Make sure versions are compatible before loading model ([#1237])
- Always require model when starting `fj-app` ([#1242])
- Fix startup delay while model is compiling ([#1244])
- Print timestamp with each status update ([#1256]; thank you, [@erenoku]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-host`

- Rename `Watcher::receive`; improve its error handling ([#1234])

#### `fj-kernel`

- Fix last known object duplication issues ([#1233], [#1238])
- Integrate all remaining objects into centralized object storage ([#1246], [#1247], [#1248], [#1252], [#1255])
- Simplify use of `MaybePartial` ([#1253])
- Consolidate builder API for `Face` in `FaceBuilder` ([#1254])


#### `fj-viewer`

- Clean up API ([#1232])

#### `fj-window`

- Simplify `fj_window::run` arguments ([#1243], [#1245])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Compile Fornjot to WebAssembly ([#1221])
- Update release procedure ([#1225])
- Run export validation on macOS ([#1226])
- Update dependencies ([#1227], [#1228], [#1229])
- Clean up some code ([#1241], [#1251])
- Update `README.md` ([#1250])


[#1221]: https://github.com/hannobraun/Fornjot/pull/1221
[#1225]: https://github.com/hannobraun/Fornjot/pull/1225
[#1226]: https://github.com/hannobraun/Fornjot/pull/1226
[#1227]: https://github.com/hannobraun/Fornjot/pull/1227
[#1228]: https://github.com/hannobraun/Fornjot/pull/1228
[#1229]: https://github.com/hannobraun/Fornjot/pull/1229
[#1232]: https://github.com/hannobraun/Fornjot/pull/1232
[#1233]: https://github.com/hannobraun/Fornjot/pull/1233
[#1234]: https://github.com/hannobraun/Fornjot/pull/1234
[#1235]: https://github.com/hannobraun/Fornjot/pull/1235
[#1237]: https://github.com/hannobraun/Fornjot/pull/1237
[#1238]: https://github.com/hannobraun/Fornjot/pull/1238
[#1241]: https://github.com/hannobraun/Fornjot/pull/1241
[#1242]: https://github.com/hannobraun/Fornjot/pull/1242
[#1243]: https://github.com/hannobraun/Fornjot/pull/1243
[#1244]: https://github.com/hannobraun/Fornjot/pull/1244
[#1245]: https://github.com/hannobraun/Fornjot/pull/1245
[#1246]: https://github.com/hannobraun/Fornjot/pull/1246
[#1247]: https://github.com/hannobraun/Fornjot/pull/1247
[#1248]: https://github.com/hannobraun/Fornjot/pull/1248
[#1250]: https://github.com/hannobraun/Fornjot/pull/1250
[#1251]: https://github.com/hannobraun/Fornjot/pull/1251
[#1252]: https://github.com/hannobraun/Fornjot/pull/1252
[#1253]: https://github.com/hannobraun/Fornjot/pull/1253
[#1254]: https://github.com/hannobraun/Fornjot/pull/1254
[#1255]: https://github.com/hannobraun/Fornjot/pull/1255
[#1256]: https://github.com/hannobraun/Fornjot/pull/1256

[@erenoku]: https://github.com/erenoku

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1021]: https://github.com/hannobraun/Fornjot/issues/1021
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
