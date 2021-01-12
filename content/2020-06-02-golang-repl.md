+++
title = "Quick prototyping with a Golang REPL"
[taxonomies]
tags = [ "dev", "go" ]
[extra]
summary = "I miss a REPL when coding in Golang. It turns out we can have a Go REPL connected to our IDE of choice."
+++

I miss a REPL when coding in Golang. It turns out we can have a Go REPL connected to our IDE of choice.

After developing in [Clojure](https://clojure.org/) for some time, I miss a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) when coding in [Golang](https://golang.org/). It turns out we can have a Go REPL connected to our IDE of choice.

{{ medium_image(
    id="1*PYNPylp2BS_XSKYIlNrXyw",
    ext="gif") }}

### Why use a REPL?

Because it allows rapid experimentation and gives feedbacks instantaneously. Will my complex regex work? Instead of double-checking everything let’s just eval one case. When selecting a substring is the last index exclusive or not? Instead of reading the documentation let’s just try it out with an example.

Data scientists usually do this with [Jupyter Notebooks](https://jupyter.org/), for example, but this is a practice which still isn’t ubiquitous in the backend/frontend/devops scene.

For more use cases, please check one video about [REPL driven development](https://www.youtube.com/results?search_query=repl+driven+development).

### How to configure it?

First install [gore](https://github.com/motemen/gore). Then integrate it with your IDE.

I’ll highlight my configuration with VSCode but other setups should be similar.

I simply added the following keybinding to “vim.visualModeKeyBindings” and “vim.normalModeKeyBindings”:

This way, whenever I type `<space>+l+t` the highlighted text will be sent to the REPL session, provided that I run `gore` in VSCode’s terminal window. If there’s no selection, the current line will be sent.

### Caveats

* gore is somewhat slow. But still is faster than running an adhoc test case + `go test` for the same purpose or setting up an adhoc entrypoint + `go run`.
* depending on the modules imported by your project it may not be trivial to replicate the same imports to your gore session.

### Does this work for other languages?

Yes! Your shell is a REPL in some sense. When writing long `for`s or commands with many pipes this may come in handy as well.

{{ medium_first(id="quick-prototyping-with-a-golang-repl-547703885bd8") }}
