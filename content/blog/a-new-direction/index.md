+++
title = "A New Direction"
date = "2023-05-15"
+++

<aside>
    A draft of this post went out to my sponsors last week. If you are a sponsor and didn't receive this, make sure you've opted in to receive my messages on GitHub Sponsors. Go to <a href="https://github.com/sponsors/hannobraun">my profile</a>, click <code>Manage</code> in the upper-right, make sure to check <code>Receive email updates from hannobraun</code>, then click <code>Update sponsorship</code>.
</aside>

I've been working on Fornjot for a long time. For a few years on the side, as a hobby. Then, since the beginning of 2022 (over a year now!), as my primary (and only) job.

I was fortunate to [find enough sponsors](https://github.com/sponsors/hannobraun) to keep this up for as long as I have. Unfortunately, my biggest sponsor decided not to renew their support after a year. As a result, since April, I have not made enough money to cover my costs.

I still have some money left in the bank, but obviously, this can't go on forever. I could keep charging ahead at full speed for as long as possible, hoping that sponsorship income will recover, but I decided that I won't. Sponsorship would have to at least double within the next 4 months or so, and I just don't think that's likely to happen.

I don't know how things will shake out long-term, but for now, I'm going to reduce my effort to one day per week (in addition to handling issues and pull request as they come up). Fornjot was already a big and ambitious project when I was working on it all week, as my primary focus. To make it realistic to achieve at all, I have to cut down its scope. Explaining how I intend to do that, is what this article is about.

It's somewhat ironic that just as most of my funding disappeared, it felt that we were finally starting to get somewhere. For many months, most of my effort went into cleanup. Finding better (simpler!) ways to do things, to get that raging technical debt back under control.

This effort turned out much more successful than I had hoped, and I've recently started to finally work on new features again. This time, built on a much more solid foundation. I want to build on that success and preserve this momentum, while cutting out everything that isn't absolutely essential to make Fornjot a useful tool.

But enough introduction, what is actually going to happen? As of now, **the goal of Fornjot is no longer to build a CAD application**. From now on, **Fornjot is a CAD kernel**, a set of Rust libraries for defining geometry, exporting it to external file formats, and viewing it in 3D.

So what does that change, in practice? Fornjot is already a code-first CAD application which uses Rust to define models. So this new direction is not a huge departure from what we already have. What it will remove is the notion of models as units that are loaded into an application at runtime. It will remove the high-level API that currently exists in the form of the [`fj`](https://crates.io/crates/fj) library. And it will remove some creature comforts, like auto-reloading models whenever you change their code.

In the future, Fornjot models will just be Rust code that uses the raw kernel API. Since the Fornjot kernel has been the main focus of my efforts so far, this isn't going to change much in terms of capability. In fact, it will provide models with much more powerful capabilities than they currently have. But it is a large departure in terms of future ambitions.

We are nearing a point were the kernel could be considered "good enough" for very basic models. From there, my focus would have shifted to the application. To making it usable in more scenarios (like within browsers), provide a more interactive editing experience, provide better insight into what the kernel is doing (for debugging purposes). None of that is going to happen now.

This allows me to keep focusing my (now much more limited) resources on kernel capabilities, giving us a realistic chance at soon having a tool that will be useful beyond just toy examples.

For all of you that were hoping for the CAD application of their dreams, I'm sorry. Maybe, with Fornjot growing into a useful CAD kernel, someone will step up to create an application around it. Maybe that someone will be me, if the funding is there. We'll see how that goes.

I'd like to end this announcement by clarifying that I'm not bitter about how things went. This is obviously not what I hoped for, initially, but it's not a bad situation. The new scope of the project is much clearer, more focused. A focus that, if I might be a bit self-critical, the project could have benefited from much earlier on.

But it is what it is. What I'm excited about, is that turning Fornjot into a useful tool for Rust developers that want to create CAD models, is an attainable short-term goal. One that's still worth my time and effort. And, I hope, still worth the efforts of contributors and the support of my sponsors.
