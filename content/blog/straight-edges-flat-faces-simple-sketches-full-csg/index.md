+++
title = "Straight edges, flat faces, simple sketches, full CSG"
date  = 2022-03-02
+++

Today I'd like to talk about the next big milestone for Fornjot, that I'm currently working towards. I've given it that long, yet weirdly catchy, name that you can see in the title. It's being [tracked on GitHub][milestone].

The goal of this milestone is to provide a minimal, yet useful, set of CAD features. It's not going to be the next big thing in the CAD world (yet!), but it will hopefully make Fornjot a useful contender for simple parts and limited use cases.

Let's deconstruct this milestone, to better understand what it's about.


### Straight edges, flat faces

An important aspect of this milestone is that the minimal feature set it provides should be *stable*. That means no weird bugs and edge cases (as far as that's possible). This can only be achieved, by limiting the feature set. Hence, only straight edges (line segments) and flat faces (polygons) will be supported.

Now, Fornjot already supports circles and can extrude them into cylinders. Will that functionality be removed? No, but these features will be marked as experimental. Users will have to specifically enable them in their model to use them, and will do so at their own risk.


### Simple sketches

The basis for all models in Fornjot are 2-dimensional sketches that are extruded into 3-dimensional solids using the sweep operation. This is already supported, but the plan is to refine that feature and make it a bit more flexible (for example, right now only sweeps along the z-axis are supported).

Currently, sketches consist only of line segments, which is actually perfectly in line with this milestones.


### Full CSG

CSG, [constructive solid geometry](https://en.wikipedia.org/wiki/Constructive_solid_geometry), is a technique for combining multiple shapes using union, difference, or intersection operations. The goal for this milestone is to have full support for CSG, for straight-edged/flat-faced shapes.

CSG is certainly not the ultimate expression of CAD modeling, but it is a useful feature. And the infrastructure that's required to make it work will be a building block for other features later.

Eventually, I want to support a more sketch-based approach to modeling, but we gotta start somewhere.


### Is that all?

So, is this it? Well, it is what I'm focused on. That doesn't mean that contributors can't work on other features, or fix bugs that fall outside the scope of this milestone.

So if you're interested in working on something else, don't be discouraged. I'm happy to assist! I just won't be able to drive such an effort myself, for the time being.


### What comes after?

What will happen after this milestone is achieved is, as of yet, undetermined. I have [plans for other milestones](https://github.com/hannobraun/Fornjot/milestones), and notes for more milestones that I still need to plan. We'll see.

At this point, I think the [Basic Usability](https://github.com/hannobraun/Fornjot/milestone/2) milestone is a strong candidate for the next focus area. As far as CAD features are concerned, I'm thinking a lot about more advanced sketching (specifically, sketching on existing surfaces) and bevels/chamfers.


### Want to help out?

Any help in getting this done is greatly appreciated! If you're interested, [head over to the milestone][milestone], pick an issue that sounds interesting to you, and get working!

Whether it's implementing a feature, fixing a bug, or testing and providing feedback, I can use the help.


[milestone]: https://github.com/hannobraun/Fornjot/milestone/1
