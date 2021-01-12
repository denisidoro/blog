+++
title = "On data-driven programming"
[taxonomies]
tags = [ "dev", "architecture" ]
[extra]
summary = "DSLs abstract away implementation details. But what are their cons and how to mitigate them?"
+++

{{ medium_cover(
    id="0*43gbACYn_imJxYko"
    caption="Photo"
    unsplash="aaronburden") }}

If there’s one thing that [functional programming](https://en.wikipedia.org/wiki/Functional_programming) has taught me is [the importance of values](https://www.youtube.com/watch?v=-6BsiVyC1kM).

Say we’re part of a startup’s founding team and are responsible for designing and building the core of its backend services. At its core, our job is to extract, transform and load values.

In general, the backend services must:

* receive values from the mobile app. e.g., a JSON payload in a REST endpoint
* perform pure logic over values. e.g., apply _f(balance, purchase)_ → _new\_balance_
* persist values in the database
* and so on

Everything else is overhead. [Getters and setters](https://www.w3schools.com/Java/java_encapsulation.asp), [Gang of Four patterns](https://springframework.guru/gang-of-four-design-patterns/) and whatnot. Sure, these abstractions may allow you to scale code or team sizes. But they are overhead, still. They’re neither inherently part of our startup’s business model nor the reason the CEO decided to hire us.

I guess that this philosophy behind values as first-class citizens led to the [code as data](https://en.wikipedia.org/wiki/Code_as_data) movement. Or [data-driven programming](https://en.wikipedia.org/wiki/Data-driven_programming). Think of DSLs and configuration files. If we want to change our app’s behavior, there’s no need to launch IntellijJ and dive into the code: we can fiddle with some JSON-like file using a notepad app. The code parses it and acts accordingly. Profit.

The benefits of such approach are undeniable, but they introduce new problems. The main ones, that I’m gonna cover in this post, are:

* entry barrier
* no type-safety

Let’s take a [Kubernetes](https://kubernetes.io/) config as an example:

<figure>
    {{ gist(url="https://gist.github.com/denisidoro/10a79e0d5b5591a5da733f07bcd25233") }}
    {% caption() %}
    Is this code correct? Is there a typo? Should `containers` be a direct child of `spec` or `selector`? I know that `ports` has a `containerPort` field, but does it have any other one? If so, is it a string or a number?
    {% end %}
</figure>

The _entry barrier_ is the time we need to take to understand this syntax. It’s a new language, in some sense. It’s good, old YAML but there are rules we need to understand prior to reading or writing such pieces of text.

By _no type-safety_ I mean all the goodies we lose that we otherwise would have in an IDE for a strongly typed language. Autocomplete, Intellisense and references to previously declared values.

For Kubernetes [I bet there are](https://dockerlabs.collabnix.com/kubernetes/kubetools/) 3761 tools to make your life easier. But what about the DSLs we create?

To illustrate, let’s suppose our job is to create a DSL/config file for a [backend-driven UI](https://medium.com/movile-tech/backend-driven-development-ios-d1c726f2913b) framework. i.e., we want to develop a system that allows front-end applications with screens and flows to be built based on backend responses. [Spotify does that](https://www.youtube.com/watch?v=vuCfKjOwZdU), by the way.

With this system, we theoretically only need one person to know the inner machinery of this code. But if a new intern wants to build a new screen, how will she know what to write in our DSL, in the first place?

### Creating an editor UI

This is in my opinion the best approach. We could have a web editor with drag-and-drop elements, drop-downs and even a live preview window that shows us how the final result would be.

Discoverability is addressed by these UI elements that pop in front of us. Correctness is guaranteed by the UI itself — by preventing us from clicking on stuff that would result into an invalid file.

But this has some cons:

* it’s expensive to write — in terms of time, at least
* it requires a full-stack team
* it doesn’t solve the problem with code review — in case the UI spits a file such as the Kubernetes one, the code review in general is limited to _LGTM_ or 🙈, as in _I trust that you used the UI correctly and I trust that the UI is correct_.

### Making use of our IDEs

What if we wrote a plugin to VSCode, for example, that somehow helps other developers write our DSLs? But wait, we’re already using a popular format such as JSON or YAML. We could instead write a plugin that helps users write JSONs that follows some convention.

But there are millions of other developers out there. I bet someone has already thought about this and took the time to do exactly what we want. I’m lazy and our job in the startup is to persist values in the database and whatnot. I, for one, don’t want to focus efforts into this.

It turns out a solution does exist out there. The first page for the Google search _typed YAML_ led me to [strictyaml](https://github.com/crdoconnor/strictyaml), which led me to [JSON Schema](https://json-schema.org/).

By using a battle-tested solution we get everything for free plus support for all major IDEs.

There are a lot of articles on the Internet about the benefits and capabilities of JSON Schema, so I won’t bother you with details. I’ll simply demonstrate it with a GIF:

{% medium_image(id="1*Em1xMQgnAbMLMU9-jnAHAg", ext="gif") %}
The intern has just joined the company, but she already knows that someone, at some point in time, wrote a `billing_address` widget.
{% end %}

The schema used in this demo was extracted from [react-jsonschema-form](https://rjsf-team.github.io/react-jsonschema-form/).

I know that having a popup window when we type sounds like a small benefit but, if you stop to think about it, the intern didn’t even have to leave the file buffer! She didn’t need to browse through the code implementation, or read documents — that are likely outdated or incomplete — or contact the people that contributed to the DSL.

[There are ways](https://github.com/redhat-developer/vscode-yaml) to extend this to YAML as well.

Oh, one suggestion: don’t write your JSON Schema _and_ your code structs/classes by hand. Let one generate the other so that there’s a single source of truth.

### Using data as code (as data)

The cool thing about having a DSL inside a JSON file is that in general it is parseable at runtime: we don’t need to deploy a new server instance and stop the previous one. We can `curl -d "@config.json" -XPOST localhost:3000/apply` to update the app's behavior.

But if we don’t need such dynamism — if e.g. in order to update this config file we need to open a PR in the same repository as the code, which leads to a new image build — then why not stay inside the code realm?

We’ll need to refactor packages a little bit so that, by convention, a given set of files will be considered as data, in contrast to implementation detail code. It seems like a small price to pay, though.

In [Kotlin](https://kotlinlang.org/) it would be [straightforward](https://medium.com/@fabiomirgo/kotlins-power-build-a-dsl-1fcf215b7bb0) to come up with a file such as this one:

<figure>
    {{ gist(url="https://gist.github.com/denisidoro/5932b86b92fc520c57fd7e07730f59eb") }}
    <figcaption>In IntelliJ, if you typed BillingAddress, an Intellisense window would show us all possible fields. If we forgot the street field, the editor would suggest us to add it.</figcaption>
</figure>

This is exactly what [Anko](https://github.com/Kotlin/anko) does in order to build Android UIs, for example.

Chances are that — assuming we wrote this in Kotlin — we have other developers in the company already familiar with Kotlin, so they will feel at home with this syntax. No extra level of indirection introduced.

{{ medium_first(id="on-data-driven-programming-925e450525e3") }}
