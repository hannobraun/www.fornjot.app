+++
title = "Weekly Release - 2022-W34"

[extra]
version = "0.12.0"
+++

After taking a week off, I'm back this week with another release! Aside from some relaxing and a lot of hiking, I spent most of my time since the last release on cleaning up and expanding the intersection testing infrastructure. Intersection testing is a necessary building block for implementing the union operation ([#42]). I [maintain a list](https://github.com/hannobraun/Fornjot/issues/42#issuecomment-1206449099) of the building blocks still required, according to my current understanding.

Meanwhile, [@Michael-F-Bryan] has upgraded/rewritten the host/model API to switch to a model-driven concept. This is invisible to users right now, but it's a prerequisite for many future improvements. [@devanlooches] and [@connor-lennox] have worked on improving the status updates that have recently been added to the Fornjot app.


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Display the last few status updates ([#919], [#945], [#952]; thank you, [@devanlooches]!)
- Add table of contents to README ([#942])
- Display model compile times in status updates ([#960]; thank you, [@connor-lennox]!)


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Add `Surface` to `Cycle` ([#939])
- Clean up and expand intersection testing code ([#940], [#941], [#946], [#947], [#948], [#949], [#950], [#951])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Switch to model-driven host API ([#885], [#934]; thank you, [@Michael-F-Bryan]!)
- Update list of sponsors in README ([#921], [#961])
- Update dependencies ([#922], [#923], [#924], [#925], [#926], [#928], [#929], [#930], [#931], [#933], [#953], [#954], [#956], [#957], [#958], [#959])
- Update release procedure ([#932])


### Issue of the Week

Fornjot is still at an early stage, and one way that manifests is in its error messages, which can be pretty raw and not very helpful. An example of that, is when a user forgets to define that a model is to be compiled to a dynamic library.

It should be able to prevent that case outright, by using a different Cargo command to compile Fornjot models, and passing the right parameters. If playing around with Rust and Cargo to get an introduction to Fornjot sounds appealing to you, why not take a look at [#938 - Use `cargo rustc` to compile models as dynamic libraries](https://github.com/hannobraun/Fornjot/issues/938)?


### Outlook

I'll be busy writing intersection testing code for the foreseeable future, as part of the process of implementing the union operation ([#42]).


[#885]: https://github.com/hannobraun/Fornjot/pull/885
[#919]: https://github.com/hannobraun/Fornjot/pull/919
[#921]: https://github.com/hannobraun/Fornjot/pull/921
[#922]: https://github.com/hannobraun/Fornjot/pull/922
[#923]: https://github.com/hannobraun/Fornjot/pull/923
[#924]: https://github.com/hannobraun/Fornjot/pull/924
[#925]: https://github.com/hannobraun/Fornjot/pull/925
[#926]: https://github.com/hannobraun/Fornjot/pull/926
[#928]: https://github.com/hannobraun/Fornjot/pull/928
[#929]: https://github.com/hannobraun/Fornjot/pull/929
[#930]: https://github.com/hannobraun/Fornjot/pull/930
[#931]: https://github.com/hannobraun/Fornjot/pull/931
[#932]: https://github.com/hannobraun/Fornjot/pull/932
[#933]: https://github.com/hannobraun/Fornjot/pull/933
[#934]: https://github.com/hannobraun/Fornjot/pull/934
[#939]: https://github.com/hannobraun/Fornjot/pull/939
[#940]: https://github.com/hannobraun/Fornjot/pull/940
[#941]: https://github.com/hannobraun/Fornjot/pull/941
[#942]: https://github.com/hannobraun/Fornjot/pull/942
[#945]: https://github.com/hannobraun/Fornjot/pull/945
[#946]: https://github.com/hannobraun/Fornjot/pull/946
[#947]: https://github.com/hannobraun/Fornjot/pull/947
[#948]: https://github.com/hannobraun/Fornjot/pull/948
[#949]: https://github.com/hannobraun/Fornjot/pull/949
[#950]: https://github.com/hannobraun/Fornjot/pull/950
[#951]: https://github.com/hannobraun/Fornjot/pull/951
[#952]: https://github.com/hannobraun/Fornjot/pull/952
[#953]: https://github.com/hannobraun/Fornjot/pull/953
[#954]: https://github.com/hannobraun/Fornjot/pull/954
[#956]: https://github.com/hannobraun/Fornjot/pull/956
[#957]: https://github.com/hannobraun/Fornjot/pull/957
[#958]: https://github.com/hannobraun/Fornjot/pull/958
[#959]: https://github.com/hannobraun/Fornjot/pull/959
[#960]: https://github.com/hannobraun/Fornjot/pull/960
[#961]: https://github.com/hannobraun/Fornjot/pull/961

[@connor-lennox]: https://github.com/connor-lennox
[@devanlooches]: https://github.com/devanlooches
[@Michael-F-Bryan]: https://github.com/Michael-F-Bryan

[#42]: https://github.com/hannobraun/Fornjot/issues/42
