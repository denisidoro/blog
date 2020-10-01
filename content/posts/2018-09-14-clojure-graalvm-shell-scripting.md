+++
title = "Using Clojure + GraalVM for shell scripting"
[taxonomies]
tags = [ "dev", "clojure", "shell" ]
[extra]
summary = "How to keep your sanity when Bash isn’t the right tool for the job, without sacrificing startup time."
+++

{{ medium_image(
   id="1*gJm3BZcNrKoPOa2QO69p3Q",
   caption="Part of a shell script written in Clojure") }}

### Motivation

Objective: develop a shell script to ease the writing of XML files.

Let’s say that the output to be generated is:

```xml
<grid>   
  <row>   
     <label id="info" />   
     <text>Now playing</text>   
  </row>   
  <row>   
     <button onTap="mute" />   
  </row>   
</grid>
```

I’d like to write it as:

```clojure
[:grid [:row [:label {:id :info}]   
             [:text "Now playing"]]   
       [:row [:button :onTap "mute"]]]
```

By the way, this syntax is a subset of [EDN](https://github.com/edn-format/edn) and called [hiccup](https://github.com/weavejester/hiccup), used by frameworks such as [re-frame](https://github.com/Day8/re-frame).

This way, in [my dotfiles](https://github.com/denisidoro/dotfiles), I can store, versionate and compose my configs as hiccup files and not as XML ones.

This is no simple task for Bash and would feel natural to use [Clojure(Script)](https://clojure.org/).

### Using Clojurescript

Until some months ago, I would write this script using [lumo](https://github.com/anmonteiro/lumo), a cross-platform, standalone ClojureScript environment.

It runs on Node.js and the V8 JavaScript engine. Also, the scripts are relatively fast: a “hello world” takes ~1s to run without caching on my machine; with caching enabled, ~0.3s.

You can find example scripts and helpers for lumo [here](https://github.com/denisidoro/dotfiles/tree/ce5cfac70858966687986614443a7e805b60df76/scripts/clojure).

### Using Clojure

With the advent of [Clojure CLI](https://clojure.org/guides/deps_and_cli) I stopped using lumo and I’m simply using the `clj` command now.

Migrating from lumo to clj is [trivial](https://clojurescript.org/about/differences) and gives us more features, such as better support for macros and multithreading.

I’m using [tools.deps](https://github.com/clojure/tools.deps.alpha) for dependency graph expansion and using [lein-tools-deps](https://github.com/RickMoynihan/lein-tools-deps) for [leiningen](https://leiningen.org/) compatibility.

The downside is the startup time: a simple “hello world” takes around ~2.5s and my XML→hiccup script, which depends on some libraries, needs more than 3s to finish.

This is fine for one-time only scripts but if I call a clj script in a Bash for-loop I’ll probably want to grab some coffee while it runs.

### Speeding up with GraalVM

[GraalVM](https://www.graalvm.org/) is a universal virtual machine for running applications written in JavaScript, Ruby, JVM-based languages and more.

By using [AOT compilation](https://www.graalvm.org/docs/reference-manual/aot-compilation/) we can compile our clj scripts to native code, which doesn’t rely on the JVM. The benefit? Let’s compare startup times:

```bash
λ echo '[:table]' | time clj -m xml 
<table /> 
clj -m xml 9.88s user 0.68s system 293% cpu 3.593 total 

λ echo '[:table]' | time native-binary 
<table /> 
native-binary 0.01s user 0.01s system 79% cpu 0.019 total
```

That’s 200x faster!

In addition, I can simply copy/paste this binary to any other machine with the same architecture + OS and it will work regardless of JVM or Node.js being installed or not.

Example scripts and helpers for clj and GraalVM can be found [here](https://github.com/denisidoro/dotfiles/tree/c4f656d6c83f34c106afc61f44568fcd4e3ea1b9/scripts/clojure).

For other people’s experiences, click [here](https://www.innoq.com/en/blog/native-clojure-and-graalvm/) or [there](https://www.astrecipes.net/blog/2018/07/20/cmd-line-apps-with-clojure-and-graalvm/).

{{ medium_first(id="https://miro.medium.com/max/700/1*gJm3BZcNrKoPOa2QO69p3Q.png" )}}
