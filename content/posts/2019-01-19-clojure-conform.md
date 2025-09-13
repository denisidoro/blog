+++
title = "clojure.spec/conform: the Fourier transform for data collections"
[taxonomies]
tags = [ "dev", "clojure" ]
[extra]
summary = "With clojure.spec/conform we can transform data in a way it's easier to reason about"
+++

Say you have received an audio sample of someone talking during the rain and you're requested to remove the ambient sound. How to do it?

```
<image missing>
```

From the spectrum above, it's difficult to perceive any pattern. However, using the Fourier transform, we can move from the time domain to the frequency domain. In this new space, it becomes easy to distinguish the sound spectrum. Analyzing the new data you would realize that the human voice frequency ranges from 100Hz to 300Hz, whereas the rain sound frequency is roughly 500Hz. After removing the undesired components, we apply the Fourier inverse transform, returning to the time domain.

```
<image missing>
```

In a sense, we had data difficult to reason about so we translated it to a simpler space, made operations on it and then went back to the initial data format.

In Clojure syntax, it could be transcribed as:

Wouldn't it be nice to do the same for data structures in general, in idiomatic code?

### A straightforward example: transforming text

Let's say you're working on a project for an autonomous car. In particular, the vehicle must break when approaching an obstacle.

You've been lucky enough to choose a Raspberry Pi for the brains, Clojurescript as the programming language and the Johnny Five library for low-level electronics.

In this contrived example, your circuit must interface with a board that periodically outputs the current state of the sensors and the actuators. In addition, you can send the desired state (speed = 10mph, for example) to this board, which in turn will apply some control? to achieve it.

```
<log missing>
```

The message format is:
```
<source>,(<key>,<value>)*

Motor,temperature,23,waterlevel,20
Radar,obst_dist,6
Speed,
```

Sure we could use some regexes, some collection mappings and then reduce the result to a CSV string. But let's solve the problem defining a grammar with clojure.spec:

```
<file missing>
```

Using clojure.spec/conform over the logs, we obtain a map  easy to reason about. This operation is our Fourier transform.

Then we make the necessary updates, like so:

Finally, we use clojure.spec/unconform to map it to another space again. In this case it wouldn't be an inverse operation but you get the idea.

### An unexpected example: writing a macro

Dynamic languages such as Clojure offer great flexibility and allow us to write code very fast. However, when reading someone's else code (or your own ~6 months+ code), it's nice to have some tips of what a function should receive or return. This help improves readability, and makes it easier to browse the code and construct a mental model of the problem.

That said, it should be easy to annotate our functions with this information, right? However, spec'ing with clojure.spec looks like this:

Even though it's arguable what is the best practice here (Clojure maintainers think that spec'ing should be done separately, possibility in another file), I think that this operation should be the most straightforward as possible. Ideally, in a syntax similar to the one proposed by plumbling/schema:

But how to write this macro? Giving support for a custom defn is very cumbersome because it has a lot of specificities: destructuring, &, multi-arities, metadata and so on.

It turns out we can use spec to write a spec-related macro. Spec-ception!
There is a nice tutorial explaining this here.

The most difficult part is to write the spec for defn. Fortunately that has already been done. We just need to make some small changes so that our macro can accept :- and the specs:

Then, we apply conform, manipulate the output map and  unconform. I'll just leave some code highlights here but hopefully you got the idea. The full code can be found here.

Finally, we can instrument the output functions and make sure everything is working as expected:

### Wrap-up

(un)conforming is an elegant approach to handling data.
