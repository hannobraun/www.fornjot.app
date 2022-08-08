+++
title = "Weekly Release - 2022-W32"
date = "2022-08-08"

[extra]
version = "0.11.0"
+++

I'm still working on the union operation ([#42]), specifically intersection code and some related cleanups. I've also been making some progress in figuring out the details of the algorithm.

Meanwhile, [@hekno25] has worked on detecting which features are provided by the graphics backend, and only using those. [@devanlooches] has added a UI element that displays the current state of the model.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Prevent crashes due to unavailable features in graphics backend ([#902], [#909], [#914]; special thanks go to first-time contributor [@hekno25]!)
- Add UI element that display current model status ([#911]; special thanks go to first-time contributor [@devanlooches]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Clean up handling of curves ([#900], [#901], [#904])
- Clean up intersection code ([#905], [#906])
- Implement face/face intersection ([#915])
- Make ray casting code public, clean it up ([#918])

#### `fj-math`

- Validate `Line` and `Circle` on construction ([#910], [#913])
- Extend and clean up `AbsDiffEq` implementations ([#912])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#892], [#894], [#895], [#896], [#899])
- Update release procedure ([#898])
- Speed up release automation ([#903])
- Update description of Cargo packages ([#916])
- Update list of sponsors in README ([#921])


### Issue of the Week

Thanks to the awesome work done by various contributors, viewing a model in Fornjot works pretty well these days! There's still room for improvement, however.

If you're interested in UI and enjoy experimenting, why not take a look at [#20 - Consider turning camera towards mouse cursor when zooming](https://github.com/hannobraun/Fornjot/issues/20)?


### Outlook

I [have made a list](https://github.com/hannobraun/Fornjot/issues/42#issuecomment-1206449099) of some of the missing building blocks for the union algorithm. That should keep busy for a while!


[#892]: https://github.com/hannobraun/Fornjot/pull/892
[#894]: https://github.com/hannobraun/Fornjot/pull/894
[#895]: https://github.com/hannobraun/Fornjot/pull/895
[#896]: https://github.com/hannobraun/Fornjot/pull/896
[#898]: https://github.com/hannobraun/Fornjot/pull/898
[#899]: https://github.com/hannobraun/Fornjot/pull/899
[#900]: https://github.com/hannobraun/Fornjot/pull/900
[#901]: https://github.com/hannobraun/Fornjot/pull/901
[#902]: https://github.com/hannobraun/Fornjot/pull/902
[#903]: https://github.com/hannobraun/Fornjot/pull/903
[#904]: https://github.com/hannobraun/Fornjot/pull/904
[#905]: https://github.com/hannobraun/Fornjot/pull/905
[#906]: https://github.com/hannobraun/Fornjot/pull/906
[#909]: https://github.com/hannobraun/Fornjot/pull/909
[#910]: https://github.com/hannobraun/Fornjot/pull/910
[#911]: https://github.com/hannobraun/Fornjot/pull/911
[#912]: https://github.com/hannobraun/Fornjot/pull/912
[#913]: https://github.com/hannobraun/Fornjot/pull/913
[#914]: https://github.com/hannobraun/Fornjot/pull/914
[#915]: https://github.com/hannobraun/Fornjot/pull/915
[#916]: https://github.com/hannobraun/Fornjot/pull/916
[#918]: https://github.com/hannobraun/Fornjot/pull/918
[#921]: https://github.com/hannobraun/Fornjot/pull/921

[@devanlooches]: https://github.com/devanlooches
[@hekno25]: https://github.com/hekno25

[#42]: https://github.com/hannobraun/Fornjot/issues/42
