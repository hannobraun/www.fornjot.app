+++
title = "Weekly Release - 2022-W29"
date = 2022-07-18

[extra]
version = "0.8.0"
+++

As previously announced, Fornjot is changing to a weekly release schedule. The previous Weekly Dev Log is being repurposed into this weekly release announcement. Otherwise not much is going to change, for now.

I've finally restarted my work on the union operation ([#42]). This has been going well so far, and I've finished a few more building blocks that are going to be needed in the finished algorithm. That work has also inspired some cleanups in `fj-kernel` and `fj-operations`, which you can see below.

Meanwhile [@jeevcat] has worked on improving the API of the `fj-viewer` crate.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Make moving the model work, even if mouse is not hovering over it ([#806])
- Make group and transform operations work on all shapes ([#825])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-interop`

- Add `ProcessedShape` from `fj-operations` ([#809]; thank you [@jeevcat]!)

#### `fj-kernel`

- Implement curve/face intersection algorithm ([#802], [#812], [#813], [#817], [#826])
- Return local curves from surface/surface intersection ([#811])
- Derive `Copy` for `VerticesOfEdge` ([#818])
- Add `Sketch`/`Solid` to distinguish between 2D/3D shapes ([#819], [#823], [#827])
- Provide more complete and convenient transform API ([#822])

#### `fj-math`

- Fix edge case in `Vector::scalar_projection_onto` ([#810])

#### `fj-operations`

- Rename `ToShape` to `Shape`; clean it up ([#820])
- Make use of `Sketch` and `Solid` ([#824])

#### `fj-viewer`

- Make events more high-level ([#803]; thank you [@jeevcat]!)


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#799], [#800], [#801])
- Update list of sponsors ([#833])


### Issue of the Week

One of Fornjot's goals is support for the web. It should be possible to embed a configurable Fornjot model in a website, where users can look at it, change the parameters, and even export it to external file formats.

We're not quite there yet. The next step would be to make sure that Fornjot can be compiled to WebAssembly. If that's something that sounds interesting to you, check out [#815 - Compile Fornjot to WebAssembly](https://github.com/hannobraun/Fornjot/issues/815).


### Outlook

My main priority remains implementing the union operation ([#42]), but I might have encountered the next detour: creating a low-level shape manipulation API for use in test suites. This is something I could use immediately, for the next step of the union algorithm implementation.

I'm currently looking into that. If I can come up with something good, it would be a huge asset for in-kernel test code, but it also has wider applications. It could even be exposed to users, as a low-level API for defining models. If it turns into too much work, I might decide to table it though, and find some workaround for my current need in the meantime.


[#799]: https://github.com/hannobraun/Fornjot/pull/799
[#800]: https://github.com/hannobraun/Fornjot/pull/800
[#801]: https://github.com/hannobraun/Fornjot/pull/801
[#802]: https://github.com/hannobraun/Fornjot/pull/802
[#803]: https://github.com/hannobraun/Fornjot/pull/803
[#806]: https://github.com/hannobraun/Fornjot/pull/806
[#809]: https://github.com/hannobraun/Fornjot/pull/809
[#810]: https://github.com/hannobraun/Fornjot/pull/810
[#811]: https://github.com/hannobraun/Fornjot/pull/811
[#812]: https://github.com/hannobraun/Fornjot/pull/812
[#813]: https://github.com/hannobraun/Fornjot/pull/813
[#817]: https://github.com/hannobraun/Fornjot/pull/817
[#818]: https://github.com/hannobraun/Fornjot/pull/818
[#819]: https://github.com/hannobraun/Fornjot/pull/819
[#820]: https://github.com/hannobraun/Fornjot/pull/820
[#822]: https://github.com/hannobraun/Fornjot/pull/822
[#823]: https://github.com/hannobraun/Fornjot/pull/823
[#824]: https://github.com/hannobraun/Fornjot/pull/824
[#825]: https://github.com/hannobraun/Fornjot/pull/825
[#826]: https://github.com/hannobraun/Fornjot/pull/826
[#827]: https://github.com/hannobraun/Fornjot/pull/827
[#833]: https://github.com/hannobraun/Fornjot/pull/833

[#42]: https://github.com/hannobraun/Fornjot/issues/42

[@jeevcat]: https://github.com/jeevcat
