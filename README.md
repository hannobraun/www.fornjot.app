# www.fornjot.app

## About

The website for [Fornjot](https://github.com/hannobraun/Fornjot), the next-generation, code-first CAD application.


## Newsletter

We're misusing Zola to also generate HTML code for the email newsletter. To generate a newsletter:

1. Add `template = "newsletter.html"` to the top of the Markdown file temporarily.
2. Run `zola build`
3. Get the HTML from the generated page in the `public/` directory.


## License

This project is open source, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with it, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md
