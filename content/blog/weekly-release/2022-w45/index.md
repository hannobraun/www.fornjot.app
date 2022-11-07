+++
title = "Weekly Release - In The Weeds"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-11-07

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.23.0"
subtitle = "Distracted by cleanups, again."
+++

I had hoped to get back on track with working on the union operation ([#42]), or rather the bug that is currently blocking that ([#1162]), but instead I got sucked deeper into cleanups. I was running into some trouble with the partial object API, which had turned into a bit of a mess in the wake of the recent robustness improvements ([#1249]), so I went off and made some improvements there.

It's probably for the better. Piling on more hacks to get the union operation done a bit quicker wouldn't make anything better. I'll still do my best to not let myself get distracted too much. Cleaning up stuff that stands in the way of what I'm working on is productive. Cleaning up stuff just because it could use some cleaning up... less so.

Meanwhile, [@zthompson47] has contributed an improvement to the compatibility check that happens when loading models.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix panic when quitting application ([#1296])
- Warn on full version mismatch of host and model ([#1300]; thank you, [@zthompson47]!)
- Improve status messages around model loading ([#1302])
- Fix panic on Windows when loading model version ([#1304], [#1308])


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Clean up partial object API ([#1294], [#1305], [#1309], [#1310], [#1312])
- Move most validation code to new validation infrastructure ([#1295], [#1299])
- Simplify `Cycle` and `Face` ([#1297])
- Improve `Debug` implementation of `Handle` ([#1298])
- Simplify `GlobalPath` transforms ([#1313])


#### `fj-viewer`

- Simplify interaction with `Gui` ([#1301])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1290], [#1291], [#1292], [#1293], [#1314], [#1315], [#1316], [#1318], [#1319])
- Upgrade to Rust 1.65.0 ([#1306])
- Make some clean-ups in internal `fj` code ([#1311])


[#1290]: https://github.com/hannobraun/Fornjot/pull/1290
[#1291]: https://github.com/hannobraun/Fornjot/pull/1291
[#1292]: https://github.com/hannobraun/Fornjot/pull/1292
[#1293]: https://github.com/hannobraun/Fornjot/pull/1293
[#1294]: https://github.com/hannobraun/Fornjot/pull/1294
[#1295]: https://github.com/hannobraun/Fornjot/pull/1295
[#1296]: https://github.com/hannobraun/Fornjot/pull/1296
[#1297]: https://github.com/hannobraun/Fornjot/pull/1297
[#1298]: https://github.com/hannobraun/Fornjot/pull/1298
[#1299]: https://github.com/hannobraun/Fornjot/pull/1299
[#1300]: https://github.com/hannobraun/Fornjot/pull/1300
[#1301]: https://github.com/hannobraun/Fornjot/pull/1301
[#1302]: https://github.com/hannobraun/Fornjot/pull/1302
[#1304]: https://github.com/hannobraun/Fornjot/pull/1304
[#1305]: https://github.com/hannobraun/Fornjot/pull/1305
[#1306]: https://github.com/hannobraun/Fornjot/pull/1306
[#1308]: https://github.com/hannobraun/Fornjot/pull/1308
[#1309]: https://github.com/hannobraun/Fornjot/pull/1309
[#1310]: https://github.com/hannobraun/Fornjot/pull/1310
[#1311]: https://github.com/hannobraun/Fornjot/pull/1311
[#1312]: https://github.com/hannobraun/Fornjot/pull/1312
[#1313]: https://github.com/hannobraun/Fornjot/pull/1313
[#1314]: https://github.com/hannobraun/Fornjot/pull/1314
[#1315]: https://github.com/hannobraun/Fornjot/pull/1315
[#1316]: https://github.com/hannobraun/Fornjot/pull/1316
[#1318]: https://github.com/hannobraun/Fornjot/pull/1318
[#1319]: https://github.com/hannobraun/Fornjot/pull/1319

[@zthompson47]: https://github.com/zthompson47

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
[#1249]: https://github.com/hannobraun/Fornjot/issues/1249
