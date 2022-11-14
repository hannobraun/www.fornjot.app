+++
title = "Weekly Release - Realization"
# TASK: Uncomment this date, once the announcement is ready to be published.
# date = 2022-11-14

# Uncomment to generate the HTML for the email newsletter.
# template = "newsletter/weekly-release.html"

[extra]
version = "0.24.0"
subtitle = "The distraction is not actually a distraction?"
+++

Had I written this paragraph a few days ago, I'd been all apologetic, explaining how I really meant to work on the bug that's currently blocking further work on the union operation ([#1162]), but how I let myself be distracted by cleanups ([#1249]). Now I realize, I've been thinking about this all wrong. Before I explain, let's recap.

Arguably, the core function of Fornjot's CAD kernel is to construct geometry in a robust and convenient way. Over recent months, I've managed to significantly increase the robustness, at the cost of convenience. The APIs we have aren't well-suited to construct geometry using the new, more robust techniques. As a result, doing so is a big pain. All code that successfully does, is messy and way more complicated than it should be.

I figured I can clean this up as I go, but this problem keeps getting in the way. Constructing geometry *is* a core function of the kernel, after all. I've realized that this isn't a distraction from what really matters. Right now, *it is what really matters*. This needs to be solved, or it will keep hanging over my head, no matter what I work on.

Starting immediately, I'm switching my focus to addressing this. I have a bunch of ideas that I can try out, and I keep getting new ones as I work on this. The goal is to create an API that supports creating geometry robustly *and* conveniently, whether in production code, or to set up a test scenario.

Meanwhile, [@kopackiw] fixed some issues in the release automation and simplified how models are passed to the Fornjot application. [@MartinKavik] added a feature that allows users to generate a new model using the Fornjot app, and added Windows support to an internal developer tool.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@ahdinosaur](https://github.com/ahdinosaur), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), [@reivilibre](https://github.com/reivilibre), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

<strong class="call-to-action">
    <p>
        If you want Fornjot to be sustainable long-term, please consider <a href="https://github.com/sponsors/hannobraun">supporting me</a> too.
    </p>
</strong>


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Don't require `--model` to pass a model ([#1323]; thank you, [@kopackiw]!)
- Add command to create a new model ([#1344]; thank you, [@MartinKavik]!)


### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Complete transition to new validation infrastructure ([#1326], [#1328], [#1330])
- Continue cleaning up partial object API ([#1331], [#1334], [#1337], [#1338], [#1339], [#1340], [#1343])

#### `fj-operations`

- Remove use of old validation infrastructure ([#1329])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1321])
- Make some minor code cleanups ([#1322], [#1332], [#1336])
- Fix some release automation issues ([#1324], [#1325], [#1333], [#1335]; thank you, [@kopackiw]!)
- Add Windows support to `export-validator` ([#1342]; thank you, [@MartinKavik]!)


[#1321]: https://github.com/hannobraun/Fornjot/pull/1321
[#1322]: https://github.com/hannobraun/Fornjot/pull/1322
[#1323]: https://github.com/hannobraun/Fornjot/pull/1323
[#1324]: https://github.com/hannobraun/Fornjot/pull/1324
[#1325]: https://github.com/hannobraun/Fornjot/pull/1325
[#1326]: https://github.com/hannobraun/Fornjot/pull/1326
[#1328]: https://github.com/hannobraun/Fornjot/pull/1328
[#1329]: https://github.com/hannobraun/Fornjot/pull/1329
[#1330]: https://github.com/hannobraun/Fornjot/pull/1330
[#1331]: https://github.com/hannobraun/Fornjot/pull/1331
[#1332]: https://github.com/hannobraun/Fornjot/pull/1332
[#1333]: https://github.com/hannobraun/Fornjot/pull/1333
[#1334]: https://github.com/hannobraun/Fornjot/pull/1334
[#1335]: https://github.com/hannobraun/Fornjot/pull/1335
[#1336]: https://github.com/hannobraun/Fornjot/pull/1336
[#1337]: https://github.com/hannobraun/Fornjot/pull/1337
[#1338]: https://github.com/hannobraun/Fornjot/pull/1338
[#1339]: https://github.com/hannobraun/Fornjot/pull/1339
[#1340]: https://github.com/hannobraun/Fornjot/pull/1340
[#1342]: https://github.com/hannobraun/Fornjot/pull/1342
[#1343]: https://github.com/hannobraun/Fornjot/pull/1343
[#1344]: https://github.com/hannobraun/Fornjot/pull/1344

[@kopackiw]: https://github.com/kopackiw
[@MartinKavik]: https://github.com/MartinKavik

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#1162]: https://github.com/hannobraun/Fornjot/issues/1162
[#1249]: https://github.com/hannobraun/Fornjot/issues/1249
