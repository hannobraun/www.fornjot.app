+++
title = "Weekly Release - UI Week"
date = 2022-10-31

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.22.0"
subtitle = "Lots of UI improvements in this one."
+++

Last week saw lots of small, user-visible improvements to the app. Anti-aliasing, various fixes to the GUI, replacing weird crashes with good error messages, and maybe most significantly, the ability to load a model from within the app, instead of having to provide it as a command-line argument (contributed by [@erenoku]).

Meanwhile my work on advancing the core CAD features is trucking along, although at a slower pace than hoped (isn't it always 😄). As a reminder, my main priority is the implementation of the union operation ([#42]), which is currently blocked by a bug ([#1162]). I've been making some progress on improving the validation infrastructure in the kernel, which should allow me to attack the bug itself soon.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix UI being blurry on some systems ([#1266]; thank you, [@erenoku]!)
- Improve error message when failing to load model ([#1268])
- Enable anti-aliasing ([#1274])
- Fix text of status messages looking jagged ([#1275])
- Fix some crashes, turn them into actionable errors ([#1276])
- Add UI to load model from within app, if no model is passed ([#1286], [#1288]; thank you, [@erenoku]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-host`

- Clean up API ([#1269])

#### `fj-kernel`

- Add new validation infrastructure ([#1279], [#1282], [#1283], [#1284], [#1285])
- Simplify handling of `MaybePartial` ([#1287])

#### `fj-math`

- Replace `Point::distance` with `distance_to` ([#1281])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1258])
- Clean up CI build ([#1259], [#1260])
- Update dependencies ([#1261], [#1262], [#1263], [#1264], [#1265], [#1267])
- Lower limits requested of the graphics backend ([#1273])
- Make sure `fj`'s `build.rs` doesn't run too often ([#1277])


[#1258]: https://github.com/hannobraun/Fornjot/pull/1258
[#1259]: https://github.com/hannobraun/Fornjot/pull/1259
[#1260]: https://github.com/hannobraun/Fornjot/pull/1260
[#1261]: https://github.com/hannobraun/Fornjot/pull/1261
[#1262]: https://github.com/hannobraun/Fornjot/pull/1262
[#1263]: https://github.com/hannobraun/Fornjot/pull/1263
[#1264]: https://github.com/hannobraun/Fornjot/pull/1264
[#1265]: https://github.com/hannobraun/Fornjot/pull/1265
[#1266]: https://github.com/hannobraun/Fornjot/pull/1266
[#1267]: https://github.com/hannobraun/Fornjot/pull/1267
[#1268]: https://github.com/hannobraun/Fornjot/pull/1268
[#1269]: https://github.com/hannobraun/Fornjot/pull/1269
[#1273]: https://github.com/hannobraun/Fornjot/pull/1273
[#1274]: https://github.com/hannobraun/Fornjot/pull/1274
[#1275]: https://github.com/hannobraun/Fornjot/pull/1275
[#1276]: https://github.com/hannobraun/Fornjot/pull/1276
[#1277]: https://github.com/hannobraun/Fornjot/pull/1277
[#1279]: https://github.com/hannobraun/Fornjot/pull/1279
[#1281]: https://github.com/hannobraun/Fornjot/pull/1281
[#1282]: https://github.com/hannobraun/Fornjot/pull/1282
[#1283]: https://github.com/hannobraun/Fornjot/pull/1283
[#1284]: https://github.com/hannobraun/Fornjot/pull/1284
[#1285]: https://github.com/hannobraun/Fornjot/pull/1285
[#1286]: https://github.com/hannobraun/Fornjot/pull/1286
[#1287]: https://github.com/hannobraun/Fornjot/pull/1287
[#1288]: https://github.com/hannobraun/Fornjot/pull/1288

[@erenoku]: https://github.com/erenoku

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
