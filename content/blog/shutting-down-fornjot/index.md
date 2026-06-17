+++
title = "Shutting Down Fornjot"
date = "2026-06-19"
+++

Fornjot is ending; unfinished, incomplete. This is not what I wanted, not why I started this project. But it is the decision I have made.

For a while now, Fornjot's development has been happening on the side, next to other work that filled most of my day. This became unsustainable, and I addressed that by serializing my efforts: focus on one project at a time until I reach a good point to pause, then switch to the next one.

Ironically, these improved conditions led to my decision to quit. Once it was Fornjot's turn, I made better progress than I could before. As a result, for the first time in a while, I was able to get my head above water and take in the bigger picture, understand where things were going.

That's when I realized I had to quit. My work was supported by sponsors. I couldn't justify taking their money, if I didn't believe that I was on a path to create something meaningfully different (and thus in some ways better) than what's already available.

It became clear to me that under current conditions, it would have easily taken another 2-3 years to put the foundation in place to even _start_ delivering such value. After already being at it for almost 6 years, I realized that I couldn't commit to that. It's too much. I no longer have it in me.

(Before writing this, I sent a private announcement to my sponsors, in which I stated that I had been working on Fornjot for about 5 years, not 6. Which is what I believed at the time, but I just checked, and it turns out the first commit to the Fornjot repository happened on July 30, 2020.)

## Mistakes I Made

Six years without even delivering something useful; it's not a good look. CAD kernels are notoriously difficult, but that's only a partial excuse. I made many mistakes along the way. In this section, I want to reflect on those mistakes. Maybe we can learn from them.

### Extrapolating from Early Success

When I started working on Fornjot, it was a departure from my earlier experiments with CAD, which had been based on signed distance fields. When I switched to boundary representation, that showed promise, and I expected linear progress from there.

That linear progress never materialized. Instead, I ran into a cliff. Discovering in so many different ways why CAD kernels, specifically b-rep kernels, are considered hard.

### Seeking Sponsorship Too Early

Starting in 2022, I became more serious about Fornjot. Opening issues and pull requests to make progress easier to follow, instead of pushing to `main` directly. Creating a website, publishing regular releases, writing release announcements. And seeking sponsorship.

It worked. The growth in sponsorship enabled everything that followed. If the project ever had a chance to succeed, it was because of that. And yet, I wouldn't do it again; not like this. I was still on my way towards creating something useful. Instead of selling a product, I had sold a dream.

That wasn't my intention. I had been expecting linear progress towards a useful tool, but then never got there. And it gnawed on me for all these years. I mean, I managed. But still, I didn't like it. I don't intend to ever sell a dream again, at least not without an actual product to deliver alongside.

### Sticking to Incremental Improvements

As I tried to scale that cliff I had run into, I limited myself to incremental improvements. Those had always served me well, while big rewrites had only ever been a distraction. Born from my yearning to replace a messy situation with a clean slate, without ever solving the underlying problems.

But both attitudes are extremes. There's fertile ground in between. Instead of convincing myself that a new idea was promising, then spending a long time implementing it in small steps, I should have been prototyping. All the time. Only then integrating, incrementally or not, what proved valuable.

### Responding to Crisis with Half-Measures

My sponsorship income had reached a sustainable level for the first time some months into 2022, due to a single, very generous sponsor. When they didn't renew in 2023, that left a large gap. A similar thing happened in 2024.

Both times I thought that sponsorship income was unlikely to recover, yet both times existing sponsors stepped up to close the gap. But there were delays between drop and recovery, which gave room for mistakes.

The first time, it broke the spell. Dissolved that image in my head, of Fornjot as the only thing I needed to do. The second time, I decided I needed to double down on diversifying and reduced Fornjot to a side project. A side project I absolutely wanted to still finish. But my life's work became something else.

And that was my biggest mistake. Each time, I had exactly two reasonable options: continue with full commitment or quit right then and there. I did neither. And if I had to trace back the project's failure to a single cause, that would be it.

### Allowing My Vision to Become Muddled

The initial goal of Fornjot was to build a code-first CAD application, which included a custom CAD kernel. Reacting to that first drop in income, I decided to cut the application and focus on the kernel only. In a way, this was good. It removed many moving parts, which simplified the project significantly.

But overall, I still think it was a mistake. An application can be focused. A CAD kernel is a generic piece of infrastructure, with many use cases to consider. At least that's how I felt about it at the time. And this muddled my vision (though it recovered somewhat, as time went on).

I should have thought more outside of the box, should have ditched those categories. Something can be a kernel, a library, but still focus on specific use cases. Be a tool instead of a building block. (And building an application on top of an existing kernel was an option that never appealed to me.)

### Prototyping Came Too Late

I eventually realized that exclusively sticking to incremental improvements was a mistake. I had maneuvered the codebase into a transitionary state, halfway between and old approach and a new one, when I realized that the new approach was unlikely to pan out.

I then started a series of experiments to discover better ideas, a better architecture to base Fornjot on. Spent over a year on that, in fact! And maybe I succeeded. I came up with a much simpler architecture, which I still think shows promise.

But I did not reap the rewards of that work. I started integrating the new architecture into the existing codebase, with the goal of replacing or adapting the old code. But I didn't finish. I ran out of steam before I got there.

## Conclusion

So there we have it. An overambitious project, addled by mistakes at almost every turn. I hope reading about it was useful to you, and maybe helps inform your own work.

As for myself, I believe I'm coming out of this smarter and stronger. But I guess we'll have to see. It's easier to state your mistakes, than it is to actually avoid repeating them. I hope that my future efforts will be proof of how much I learned.
