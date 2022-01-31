+++
title = "Weekly Dev Log for 2022-W04"
date  = 2022-01-31
+++

In addition to the work highlighted below, last week saw the release of Fornjot 0.5. Check out the [release announcement](/blog/fornjot-0-5-0/)!

### Sponsors

Fornjot is supported by [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you're interested in helping to make the project sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### Added and improved

- Improve performance and reliability of circle approximation ([#111], [#112]; special thanks to first-time contributor [@gzsombor](https://github.com/gzsombor))


### Bugs fixed

- Fix floating point accuracy issue in triangulation ([#81])
- Fix model loading error, if name contains '-' ([#107])


### Documentation

- Improve README ([#84], [#85], [#99])
- Improve documentation of `fj` crate ([#86], [#106])
- Fix typo in "star" README ([#94])
- Add model to test/demonstrate disjoint unions ([#108])


### Internal improvements

- Update dependencies ([#75], [#76])
- Add contribution guide ([#80], [#83])
- Remove "enclosure" model ([#87])
- Publish version 0.5.0 ([#89], [#90])
- Document release procedure ([#93])
- Remove `TASK`s from source code ([#96])
- Merge some refactorings from ongoing development work ([#98], [#109])


### Website

- Move website to separate repository ([#77], [#79])
- Update base URL ([#3])
- Add blog posts ([#4], [#9], [#11])
- Update community page ([#5], [#10])
- Tweak design ([#6])
- Add template to generate email newsletter ([#7])
- Add missing country to address ([#8])
- Add link to GitHub repository to navigation ([#12])
- Replace screenshot with "star" screenshot ([#13])


### Outlook

Right now I'm busy working on [#78], which is a refactoring that should enable [#97], which is another refactoring that will hopefully enable constructive solid geometry ([#42], [#43], [#44]). So lots of cleanup work ahead, but I think it's important to do this right. Anything else would just result in more bugs, and more work in the end.


[#75]: https://github.com/hannobraun/Fornjot/pull/75
[#76]: https://github.com/hannobraun/Fornjot/pull/76
[#77]: https://github.com/hannobraun/Fornjot/pull/77
[#79]: https://github.com/hannobraun/Fornjot/pull/79
[#80]: https://github.com/hannobraun/Fornjot/pull/80
[#81]: https://github.com/hannobraun/Fornjot/pull/81
[#83]: https://github.com/hannobraun/Fornjot/pull/83
[#84]: https://github.com/hannobraun/Fornjot/pull/84
[#85]: https://github.com/hannobraun/Fornjot/pull/85
[#86]: https://github.com/hannobraun/Fornjot/pull/86
[#87]: https://github.com/hannobraun/Fornjot/pull/87
[#89]: https://github.com/hannobraun/Fornjot/pull/89
[#90]: https://github.com/hannobraun/Fornjot/pull/90
[#93]: https://github.com/hannobraun/Fornjot/pull/93
[#94]: https://github.com/hannobraun/Fornjot/pull/94
[#96]: https://github.com/hannobraun/Fornjot/pull/96
[#98]: https://github.com/hannobraun/Fornjot/pull/98
[#99]: https://github.com/hannobraun/Fornjot/pull/99
[#106]: https://github.com/hannobraun/Fornjot/pull/106
[#107]: https://github.com/hannobraun/Fornjot/pull/107
[#108]: https://github.com/hannobraun/Fornjot/pull/108
[#109]: https://github.com/hannobraun/Fornjot/pull/109
[#111]: https://github.com/hannobraun/Fornjot/pull/111
[#112]: https://github.com/hannobraun/Fornjot/pull/112

[#42]: https://github.com/hannobraun/Fornjot/issues/42
[#43]: https://github.com/hannobraun/Fornjot/issues/43
[#44]: https://github.com/hannobraun/Fornjot/issues/44
[#78]: https://github.com/hannobraun/Fornjot/issues/78
[#97]: https://github.com/hannobraun/Fornjot/issues/97

[#3]: https://github.com/hannobraun/www.fornjot.app/pull/3
[#4]: https://github.com/hannobraun/www.fornjot.app/pull/4
[#5]: https://github.com/hannobraun/www.fornjot.app/pull/5
[#6]: https://github.com/hannobraun/www.fornjot.app/pull/6
[#7]: https://github.com/hannobraun/www.fornjot.app/pull/7
[#8]: https://github.com/hannobraun/www.fornjot.app/pull/8
[#9]: https://github.com/hannobraun/www.fornjot.app/pull/9
[#10]: https://github.com/hannobraun/www.fornjot.app/pull/10
[#11]: https://github.com/hannobraun/www.fornjot.app/pull/11
[#12]: https://github.com/hannobraun/www.fornjot.app/pull/12
[#13]: https://github.com/hannobraun/www.fornjot.app/pull/13
