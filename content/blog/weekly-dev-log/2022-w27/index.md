+++
title = "Weekly Dev Log - 2022-W27"
date  = 2022-07-11
+++

My efforts last week were dominated by releasing [version 0.7](/blog/fornjot-0.7/) of Fornjot. Getting out a release after 2 months of frantic work is difficult. Writing the changelog and the release announcement was lots of work!

This time, I've decided I've had enough. Starting next week, there will be weekly releases, and the changelog format will be updated to closely match this Weekly Dev Log, which will become the release announcement. This will hopefully make releases both regular and manageable.

After the release, I took a bit of a breather and worked on some smaller improvements to the `fj` crate.

Meanwhile, contributors have been busy! I've merged an experimental GUI based on [egui](https://github.com/emilk/egui), submitted by [@follower], which should provide the basis for more UI improvements in the future! [@jeevcat] has been busy improving the zooming behavior, and implementing lots of improvements to the project's development infrastructure.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Add experimental egui-based UI ([#763]; special thanks to first-time contributor [@follower]!)
- Improve zooming behavior ([#764], [#781]; thanks to [@jeevcat]!)
- Update wording in README ([#795])
- Subsume `fj::Circle` into `fj::Sketch` ([#796])
- Rename `#[value]` attribute to `#[param]` ([#797])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

- None this week. Busy getting out the release!


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update list of sponsors ([#765])
- Fail CI build, if doc warnings are present ([#766], [#774], [#776], [#780]; thanks to [@jeevcat]!)
- Update dependencies ([#767], [#768], [#769], [#770], [#771], [#775])
- Update Rust version to 1.62.0 ([#777])
- Check that changes to `Cargo.lock` are committed ([#782]; thanks to [@jeevcat]!)
- Release version 0.7.0 ([#779], [#784], [#785], [#786], [#787], [#789], [#790], [#791])
- Update release procedure ([#792])


### Issue of the Week

We recently got experimental support for [egui](https://github.com/emilk/egui), which should make future extensions to our UI much easier. This was a big step forward, and to get it merged without further delay, I decided to accept a trade-off: weakening the previously clear separation between `fj-window` and `fj-viewer`.

`fj-viewer` is a library for displaying Fornjot models. `fj-window` had previously been extracted from it to cleanly separate `fj-viewer` from the windowing library used. `fj-window` uses winit, and the split allowed alternative applications that use something else to still use `fj-viewer`.

Unfortunately, the addition of egui re-added a dependency on winit to `fj-viewer`, indirectly through egui. If you have some experience with winit and egui, or are interested to learn, why not look into [#793 - `fj-viewer` depends on winit](https://github.com/hannobraun/Fornjot/issues/793)?


### Outlook

The release is out of the way, and I'm looking forward to restart my work on implementing the union operation ([#42])!


[#763]: https://github.com/hannobraun/Fornjot/pull/763
[#764]: https://github.com/hannobraun/Fornjot/pull/764
[#765]: https://github.com/hannobraun/Fornjot/pull/765
[#766]: https://github.com/hannobraun/Fornjot/pull/766
[#767]: https://github.com/hannobraun/Fornjot/pull/767
[#768]: https://github.com/hannobraun/Fornjot/pull/768
[#769]: https://github.com/hannobraun/Fornjot/pull/769
[#770]: https://github.com/hannobraun/Fornjot/pull/770
[#771]: https://github.com/hannobraun/Fornjot/pull/771
[#774]: https://github.com/hannobraun/Fornjot/pull/774
[#775]: https://github.com/hannobraun/Fornjot/pull/775
[#776]: https://github.com/hannobraun/Fornjot/pull/776
[#777]: https://github.com/hannobraun/Fornjot/pull/777
[#779]: https://github.com/hannobraun/Fornjot/pull/779
[#780]: https://github.com/hannobraun/Fornjot/pull/780
[#781]: https://github.com/hannobraun/Fornjot/pull/781
[#782]: https://github.com/hannobraun/Fornjot/pull/782
[#784]: https://github.com/hannobraun/Fornjot/pull/784
[#785]: https://github.com/hannobraun/Fornjot/pull/785
[#786]: https://github.com/hannobraun/Fornjot/pull/786
[#787]: https://github.com/hannobraun/Fornjot/pull/787
[#789]: https://github.com/hannobraun/Fornjot/pull/789
[#790]: https://github.com/hannobraun/Fornjot/pull/790
[#791]: https://github.com/hannobraun/Fornjot/pull/791
[#792]: https://github.com/hannobraun/Fornjot/pull/792
[#795]: https://github.com/hannobraun/Fornjot/pull/795
[#796]: https://github.com/hannobraun/Fornjot/pull/796
[#797]: https://github.com/hannobraun/Fornjot/pull/797

[#42]: https://github.com/hannobraun/Fornjot/issues/42

[@follower]: https://github.com/follower
[@jeevcat]: https://github.com/jeevcat
