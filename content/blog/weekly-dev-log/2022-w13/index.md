+++
title = "Weekly Dev Log - 2022-W13"
date  = 2022-04-04
+++

It's been a slow week, as I was busy dealing with the fallout from my broken office PC. Good news is, I managed to get my data (including some Fornjot work-in-progress) off of it, and managed to pick up right where I left off the week before, making some progress on [#105].

In other news, [@hendrikmaus] continued his work of automating the release process. I hope to test this out soon, as a new release is overdue, in my opinion.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to the Fornjot application, the API for creating models, and documentation.

- Improve documentation of `fj` crate ([#411])
- Make some cleanups in `fj` API ([#412])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Simplify `Handle` ([#413])
- Replace `add_*` methods with `Shape::insert` ([#416])
- Add `Shape::get_handle`, `Shape::get_handle_or_insert` ([#417])
- Extend builder API, use it to simplify approximation tests ([#418])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#405], [#406], [#407], [#410])
- Improve release infrastructure ([#408], [#409]; thanks to [@hendrikmaus]!)
- Simplify `Store` ([#414])
- Clean up shape validation internals ([#415])


### Issue of the Week

Do you enjoy playing with unsafe Rust code? There's a tiny bit of it in Fornjot, and while it seems to be sound, it's leaking memory!

If that sounds like a fun introduction to Fornjot to you, maybe give [#186 - `fj::Sketch` is leaking memory](https://github.com/hannobraun/Fornjot/issues/186) a try!


### Outlook

Now that I'm back in my groove, I expect some more progress on [#105]. I *think* it shouldn't take too much longer to wrap that up, but of course there's always the possibility for surprises. Next, I plan to continue [#141], which I hope I can finish quickly.

After that, I plan to put out a new release of all Fornjot crates. It's been a while since the last one!


[#405]: https://github.com/hannobraun/Fornjot/pull/405
[#406]: https://github.com/hannobraun/Fornjot/pull/406
[#407]: https://github.com/hannobraun/Fornjot/pull/407
[#408]: https://github.com/hannobraun/Fornjot/pull/408
[#409]: https://github.com/hannobraun/Fornjot/pull/409
[#410]: https://github.com/hannobraun/Fornjot/pull/410
[#411]: https://github.com/hannobraun/Fornjot/pull/411
[#412]: https://github.com/hannobraun/Fornjot/pull/412
[#413]: https://github.com/hannobraun/Fornjot/pull/413
[#414]: https://github.com/hannobraun/Fornjot/pull/414
[#415]: https://github.com/hannobraun/Fornjot/pull/415
[#416]: https://github.com/hannobraun/Fornjot/pull/416
[#417]: https://github.com/hannobraun/Fornjot/pull/417
[#418]: https://github.com/hannobraun/Fornjot/pull/418

[#105]: https://github.com/hannobraun/Fornjot/issues/105
[#141]: https://github.com/hannobraun/Fornjot/issues/141

[@hendrikmaus]: https://github.com/hendrikmaus
