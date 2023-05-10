+++
title = "The world needs another CAD program"
date  = 2022-02-23

# Workaround, since we can't tell a transparent section that we don't want it as
# part of the URL.
path = "blog/the-world-needs-another-cad-program"

[extra]
canonical = "https://hanno.braun-odw.eu/notes/the-world-needs-another-cad-program/"
+++

You might be looking at the huge line-up of CAD software that is available today and think, what's the point of building another one? Aren't there already enough options?

Yes, there are a whole lot of options to choose from. And I tried to do that, again and again. Choose, I mean. But I've always come away unhappy from the experience.

So what's the problem? Why can't I find a CAD program that I like? Let's go through the list of problems, as I see them.


### 1. It doesn't run on Linux.

Also known in its more restrictive form, "it only runs on Windows".

I've been using Linux for almost two decades now. From time to time, I've been running Windows on the side, but that has never worked out well.

Switching between two systems regularly is a pain, and inefficient. If you don't use Windows often enough, that's a pain too, because then you're spending more time updating and rebooting than doing actual work. A virtual machine is better in some ways, but comes with its own problems.

And listen, I'm not going to argue that Linux is "better". But it is *better for me*, and many other people as well. The software I usually run tends to work better on Linux than it does on Windows (or macOS). And I understand how it works, at least well enough to fix the problems I face.

In any case, this takes away most of the commercial CAD programs, except for [OnShape](https://www.onshape.com/) and a bunch of others that don't seem suitable to the kind of work that I'd like to do. (If you know of another option that I should take a look at, [please let me know](mailto:hanno@braun-odw.eu).)


### 2. It's too expensive.

CAD software tends to cost a lot. That wouldn't be an issue, if I was making a substantial amount of money from using it, but I don't. I'm mainly a software developer, who dabbles with making physical products on the side.

Yes, I know lots of options are free for non-commercial use, but that doesn't count. There is lots of interesting space between "hobbyist" and "can justify to spend loads of money", and I tend to find myself right in the middle of that.

At this point, most of what's left is open source. Works for me, I prefer open source software anyway. Let's continue.


### 3. It is GUI-based.

I know this one is controversial, but I think CAD should be code-first. [I've already written about that](/blog/code-cad-advantages), so head over to that article, if you want to learn more.

Here, we lose [FreeCAD](https://www.freecadweb.org/), [SolveSpace](https://solvespace.com/) (both which I use from time to time), and a bunch of others.


### 4. It only supports a basic feature set.

The most popular Code-CAD application, as far as I can tell, is [OpenSCAD](https://openscad.org/). While programming your CAD models, like you would program software, provides a lot of flexibility, the feature set of OpenSCAD is otherwise very limited. This makes many CAD modeling tasks tedious.

I want to use something more powerful, and that rules out OpenSCAD and a whole bunch of other less-known alternatives.


### 5. There's room for innovation.

At this point, we're down to a number of less popular solutions that are based on [OCCT](https://www.opencascade.com/open-cascade-technology/). I'm aware of [CadQuery](https://cadquery.readthedocs.io/en/latest/intro.html), [Cascade Studio](https://zalo.github.io/CascadeStudio/), and [pythonOCC](https://github.com/tpaviot/pythonocc).

These look great, but I think there's a lot of room for innovation:

- **Experimentation in CAD kernels:** OCCT ist just one CAD kernel. I think it's worth taking a fresh look at the problem, using a different (some might say better) language, like [Rust](https://www.rust-lang.org/).
- **More diverse selection of modeling language:** All the OCCT-based options mentioned above use either Python or JavaScript, which are all dynamically typed. While using a statically typed language wouldn't be better per se, it would be nice to have a wider range of options. Different languages are good for different use cases.
- **Broader platform support:** Each of the options that are left either run on desktop platforms, or in the browser. It would be nice to have a CAD program that supports all major desktop and mobile platforms, as well as the web. We have the technology now to make that practical.


### Conclusion

I've presented the problems I have with currently available CAD software, how they rule out the large majority of it. The few CAD programs that most of these problems don't apply to are pretty similar to each other, each being based on the same CAD kernel, and each using a dynamic language for modeling.

I think there's room for something different. A CAD program that:

- uses a code-first approach;
- is open source;
- is broadly available, with support for all major platforms;
- is based on a new CAD kernel, written in Rust;
- provides a well-rounded feature set;
- and a selection of different modeling languages.

This is what I'm trying to do with Fornjot. If you think this is a worthwhile endeavor, [please consider supporting me](https://github.com/sponsors/hannobraun).
