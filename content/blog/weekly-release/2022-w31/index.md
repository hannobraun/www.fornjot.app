+++
title = "Weekly Release - 2022-W31"
date = "2022-08-01"

[extra]
version = "0.10.0"
+++

I was able to spend most of last week heads-down working on the union operation ([#42]). The bulk of this work related to the various intersection tests that the union algorithm requires, but it also resulted in some cleanups to kernel APIs.

[@Michael-F-Bryan] has been busy too, with the `--version` argument he added to `fj-app` being merged last week.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Add `--version` argument ([#868]; thank you, [@Michael-F-Bryan]!)
- Improve README ([#877], [#882])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-interop`

- Re-use `fj_math::Triangle` in `fj_interop::mesh` ([#886])

#### `fj-kernel`

- Improve wording in doc comment ([#880])
- Clean up API of object types ([#881], [#891])
- Implement curve/edge intersection ([#884], [#888], [#889])
- Clean up surface/surface intersection ([#890])

#### `fj-math`

- Make `Triangle::from_points` fallible; add `Line::is_coincident_with` ([#887])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#870], [#871], [#872], [#873], [#874], [#876])
- Update release procedure ([#875], [#879])


### Issue of the Week

The Fornjot repository has a continuous deployment workflow that builds binaries for the Fornjot app every time new changes are pushed to the `main` branch. There is currently a bug that mislabels those binaries as official release binaries, which results in the wrong output being displayed for them, if the user calls the app with the `--version` argument.

If you're looking for a way to get involved with Fornjot and are interested in CI/CD and GitHub Actions, why not take a look at [#883 - All binaries built by CD workflow are labeled as release binaries](https://github.com/hannobraun/Fornjot/issues/883)?


### Outlook

I understand the union algorithm from a high-level (and have been for a while now, I think), but working out and implementing all the details is still a challenge. I expect that this work will continue for a while.


[#868]: https://github.com/hannobraun/Fornjot/pull/868
[#870]: https://github.com/hannobraun/Fornjot/pull/870
[#871]: https://github.com/hannobraun/Fornjot/pull/871
[#872]: https://github.com/hannobraun/Fornjot/pull/872
[#873]: https://github.com/hannobraun/Fornjot/pull/873
[#874]: https://github.com/hannobraun/Fornjot/pull/874
[#875]: https://github.com/hannobraun/Fornjot/pull/875
[#876]: https://github.com/hannobraun/Fornjot/pull/876
[#877]: https://github.com/hannobraun/Fornjot/pull/877
[#879]: https://github.com/hannobraun/Fornjot/pull/879
[#880]: https://github.com/hannobraun/Fornjot/pull/880
[#881]: https://github.com/hannobraun/Fornjot/pull/881
[#882]: https://github.com/hannobraun/Fornjot/pull/882
[#884]: https://github.com/hannobraun/Fornjot/pull/884
[#886]: https://github.com/hannobraun/Fornjot/pull/886
[#887]: https://github.com/hannobraun/Fornjot/pull/887
[#888]: https://github.com/hannobraun/Fornjot/pull/888
[#889]: https://github.com/hannobraun/Fornjot/pull/889
[#890]: https://github.com/hannobraun/Fornjot/pull/890
[#891]: https://github.com/hannobraun/Fornjot/pull/891

[#42]: https://github.com/hannobraun/Fornjot/issues/42

[@Michael-F-Bryan]: https://github.com/Michael-F-Bryan
