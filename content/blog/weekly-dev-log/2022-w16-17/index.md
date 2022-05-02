+++
title = "Weekly Dev Log - 2022-W16/W17 (Post-Vacation Edition)"
date  = 2022-05-02
+++

I'm back from vacation, and feeling eager to get back into Fornjot! Despite my vacation, the last two weeks were not entirely uneventful. I picked off some smaller issues, and [@freylint] started contributing by updating some API documentation.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix circles not being rotatable ([#493])
- Fix word in doc comment ([#502])
- Make rotation work without a focus point ([#503])
- Support sweeping in arbitrary directions ([#505])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Clean up approximation code ([#496])

#### `fj-math`

- Make `fj_math::Transform` more flexible ([#501])
- Make some small improvements ([#504])

#### `fj-viewer`

- Add missing documentation ([#487], [#492]; special thanks to first-time contributor [@freylint]!)


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#488], [#489], [#499], [#500])
- Remove unused C header files ([#495])
- Ignore more IDE configuration ([#498]; thanks to [@freylint]!)
- Add names for all CI build steps ([#507])


### Issue of the Week

API documentation is important, but ours contains some unnecessary mistakes. If you're interested in helping out, why not take a look at [#271 - Check documentation in CI build](https://github.com/hannobraun/Fornjot/issues/271)? This will help improve the quality of our documentation.


### Outlook

After taking a look at some smaller things that have accumulated over the last two weeks, my next focus is going to be a new release. I've waited way too long, so this will require a release announcement of epic proportions. Other than that, I hope that it won't be too much of an issue.


[#487]: https://github.com/hannobraun/Fornjot/pull/487
[#488]: https://github.com/hannobraun/Fornjot/pull/488
[#489]: https://github.com/hannobraun/Fornjot/pull/489
[#492]: https://github.com/hannobraun/Fornjot/pull/492
[#493]: https://github.com/hannobraun/Fornjot/pull/493
[#495]: https://github.com/hannobraun/Fornjot/pull/495
[#496]: https://github.com/hannobraun/Fornjot/pull/496
[#498]: https://github.com/hannobraun/Fornjot/pull/498
[#499]: https://github.com/hannobraun/Fornjot/pull/499
[#500]: https://github.com/hannobraun/Fornjot/pull/500
[#501]: https://github.com/hannobraun/Fornjot/pull/501
[#502]: https://github.com/hannobraun/Fornjot/pull/502
[#503]: https://github.com/hannobraun/Fornjot/pull/503
[#504]: https://github.com/hannobraun/Fornjot/pull/504
[#505]: https://github.com/hannobraun/Fornjot/pull/505
[#507]: https://github.com/hannobraun/Fornjot/pull/507

[@freylint]: https://github.com/freylint
