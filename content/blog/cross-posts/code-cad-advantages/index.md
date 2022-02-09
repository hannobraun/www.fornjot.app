+++
title = "Advantages of Code-CAD"
date  = "2022-02-09"

# Workaround, since we can't tell a transparent section that we don't want it as
# part of the URL.
path = "blog/code-cad-advantages"

[extra]
canonical = "https://hanno.braun-odw.eu/notes/code-cad-advantages/"
+++

I'm a software developer. I've learned to prefer clear code, written in a programming language, to graphical user interfaces that control opaque data formats. Applied to CAD, this approach is called Code-CAD or Programmatic CAD. In this note, I'm presenting the advantages of Code-CAD, as I see them.

### Source of Truth

Let's start with one advantage that may be easy to overlook: If code is the source of truth for your model, you can inspect and manipulate this source of truth directly.

With typical CAD software, the model is defined by a data format that you are walled off from. Most likely, you never interact with it directly. Your interaction is filtered through a complicated graphical user interface (GUI).

If that interface is well-made and bug-free, you might not perceive this as a disadvantage. But if you use a graphical application extensively, you will more than likely, sooner or later, run into a bug that messes up your data in a way that the software may or may not be able to recover from.

And while software development comes with lots of its own headaches, this specific problem is typically not one of them.


### Inherent Advantages of Programming

Simply by using a programming language, you get some inherent advantages:

- A well-written model is going to be parametric by nature. If you decide to change some decision you made early on in the modeling process, you won't have to modify a lot of code.
- Programming languages allow you to build abstractions. Write one piece of code to generate a gear, another one on top of that to generate the whole gearbox.
- You get the full power of the language. Compute the number and positions of support beams based on the load they have to carry. Or generate the complex geometry of a key from a simple numeric representation.
- Programming languages allow for comments. You can explain your model and document your design intent, in as much detail as required.

A sufficiently advanced GUI-based CAD program can also provide all of these things. However, the developers of that program might not have gotten around to develop what you need yet. And even if they have, they could have used that time to develop something else that would have been just as useful.

Many CAD programs can be extended with plugins, which provides the full power of a programming language in another way. But there will always be a disconnect between GUI-based CAD modeling and writing a plugin. On the other hand, if you've been writing code anyway, just to create your model, then writing some slightly more complicated code poses not much a hurdle.


### Using Tools Made for Text

The large majority of programming languages are represented (and hence written) as simple text. By using such a language, you can tap into a vast ecosystem of tools that have been developed to deal with text:

- You can use a text editor or IDE to view and manipulate it. Modern IDEs provide advanced tooling to help you do that, like allowing you to change the name of a module automatically, all across your code. Or jumping to the definition of a function from where it is used.
- Using standard version control software, you can track changes to your code and note the reasons you made these changes. This makes it straight-forward to track how a model has changed, or return to an older version if you took a wrong turn somewhere.
- The same version control software enables you to collaborate with others, worldwide. Sending your code to a team member, receiving their changes, and talking about those changes.

Again, a sufficiently advanced GUI-based CAD program can provide you with all of these capabilities. But the developers have to put in the effort to make it happen. And their solutions might not compare favorably with the standard tools from the software development world, which have been refined over years and decades.


### The Disadvantage

There is one glaring disadvantage, of course: To use Code-CAD, you have to know how to write code. And while that is a very useful skill to have in general, not everyone wants to invest the effort into learning that.

This is somewhat offset by the fact that learning a non-trivial GUI-based CAD application is also not easy. But if you're already proficient in a CAD program that you're reasonably happy with, learning a completely new skill to switch to something else might not be the best use of your time.

And that's fine, of course. The best solution to a problem depends not only on the problem itself, but on the context as well. Your context is going to be different from mine, so what's the best solution for me, might not even be a good one for you.


### A Hybrid Approach

There are other disadvantages to Code-CAD, of course. Making a sketch and applying some constraints can be very easy with a nice GUI, while having to type it all out would be very tedious.

For that reason, I think the ideal CAD program would use a hybrid approach: Being code-first for all the reason presented above, but letting you edit that code through graphical tools, where it makes sense. I'm not aware of any system that works like that, at least to the extent that is possible in principle. I believe that creating such a system would be a worthwhile effort.
