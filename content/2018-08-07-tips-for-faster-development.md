+++
title = "Save 15min a day: tips for faster development"
[taxonomies]
tags = [ "dev", "productivity", "git", "shell" ]
[extra]
summary = "Practices I've adopted for git, terminal, shell scripting and more that allow me to go home a little bit earlier."
+++

{{ medium_cover(
    id="0*v2N2qF1dh5l_Gqgr",
    caption="An hourglass with most of its sand on its bottom", 
    unsplash="neonbrand") }}

I’m lazy!

Working is satisfying but if I were to choose between, say, fixing broken integration tests vs watching Netflix, swimming or spending some quality time with my girlfriend, I would probably choose the latter.

However, the job market is increasingly competitive. Companies expect you to deliver results as most as possible. In order to catch up with colleagues, employees tend to work until late at night. [Impostor syndrome](https://en.wikipedia.org/wiki/Impostor_syndrome) isn’t unusual. Even so, people are fired for performance reasons. There’s simply no space for not shipping at the end of the day.

Unfortunately, as much as I don’t want to work 10h/day, it’s expected from me to get the job done. So what I try to do is to code the most I can during the hours I’m in the office.

Do I know the secrets for productivity? Absolutely not!   
Nevertheless, during these years I have developed production-ready code, my productivity has evolved a little bit.  
The purpose of this post isn’t simply about showing what I learned. Instead, it’s about showing examples that will possibly trigger your mind so that you yourself can improve your workflow.

If after this post you save 15 minutes at the end of the day, my objective here will be accomplished.

Without further ado, let’s get started with the first tip!

### File/folder navigation

I think it’s fundamental to navigate around your file system.  
As an exercise, let’s _cd_ to a mobile app from work.   
This is a simple, tedious and time-consuming way to do it.

```bash
cd ~/dev/work/mobile
```

Creating a simple **alias function** already saves us some keystrokes:

```bash
cdw() { cd "$HOME/dev/work/$1"; }  
cdw mobile
```

But we still can do much better. An improved version of this could be achieved using the [**fzf command**](https://github.com/junegunn/fzf). fzf is a fuzzy-finder that allows you to interactively choose an input from _stdin_. I use it for everything and folder browsing isn’t an exception.

{{ gist(
    url="https://gist.github.com/denisidoro/5485fe0f1d51922559366775f2f2904d",
    caption="We pipe the list of relevant folders to fzf and simply call a cd command with the result") }}

So, in a single command we get for free: listing, a nice interface and matching by substrings. We also added another arity to our command that _cd_s to the best match directly. Nice, isn’t it?

A more generic approach would be using the [**fasd command**](https://github.com/clvv/fasd). Fasd keeps track of every folder you _cd_ into and scores them by frecency: a metric based on frequency and recency, so it knows where you probably wanna go to.  
This way, if I have already visited the mobile folder, I can simply type ‘’mob’’ and I’m done.

```bash
~/Downloads  
λ j mob

~/dev/work/mobile  
λ
```

From this, we can come up with a lot of interesting commands.  
Using [this command](https://github.com/denisidoro/dotfiles/blob/bebfcbd6c32e7a5a9dc151d6a9153ae010d8d32f/shell/aliases/browsing.zsh#L100), for example, I can quickly open in [vim](https://github.com/neovim/neovim) any file that is inside any subfolder of the folder I’m currently in. Likewise, if I input a substring to it, it opens the best match directly.

{{ medium_image(
    id="1*IeUQKixUrBM5xHp1vC_gog", ext="gif",
    caption="With simple scripts you don’t even need to launch a project in an IDE to quickly edit a file" )}}

A nice addition is [**ranger**](https://github.com/ranger/ranger), a file explorer for the terminal: we can quickly switch between folders, see previews and much more.

{{ medium_image(
    id="1*NILX79uV_jiYHh7AYeI-Dw",
    caption="Ranger displays multiple columns so that you can view parent folders as well as file contents") }}

Finally, I really recommend having an easy way to **switch between** the **terminal** and your OS’s **explorer.** Here, I’m using [Alfred](https://www.alfredapp.com/) to open the current folder in Finder in the terminal.

{{ medium_image(
    id="1*Pf1KlZXto2hEQe1lGZlPQg",
    caption="With Alfred, I can quickly launch a terminal window in the same folder as Finder") }}

### Git

The most straightforward tip is to use [**shell aliases**](https://gist.github.com/mwhite/6887990).  
So, instead of typing the following…

```bash
git checkout master  
git pull  
git checkout feature  
git merge master  
git commit -a
```

…we can save lots and lots of keystrokes (and time) by using:

```bash
gcom  
gl  
gco -  
gmm  
gca
```

Another tip is to use a [**global .gitignore**](https://help.github.com/articles/ignoring-files/).  
This way, you have the guarantee that you’ll never ever commit again _node\_modules_, IDE files or sensitive data, regardless of your repository’s .gitignore.

[**Git hooks**](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks) are very powerful as well. They are scripts that are run before you commit or push, for example.  
How many times have you committed a bad JSON, for example, then committed a “fix typo” immediately after? Git hooks for the rescue!  
Simply validate JSON files in a pre-commit hook: if anything is incorrect, the commit may be aborted.

{{ medium_image(
    id="1*UrZP9iLjSNNClCvahNV3qg", 
    ext="gif",
    caption="A git hook may prevent us from commiting bad code") }}

I also use this for other file syntax validation as well as for preventing me from committing sensitive data, such as AWS keys.

Write your **own git helpers**. This is how I git checkout:

{{ medium_image(
    id="1*Q2yCC9Dp730GcpXDXJsQ7A", 
    ext="gif", 
    caption="Remember fzf? I’m using it to select available branches") }}

Using [**git squash**](https://git-scm.com/book/en/v2/Git-Tools-Rewriting-History)  really changed my daily work.  
First, let’s see what many people consider a nice, organized PR:

```
6c96a Add logic  
84f15 Replace login screen  
7960c Add button callbacks to new login screen  
22cd3 Add ...  
6c96a Add unit tests  
34fa5 Fix unit tests  
12cd4 Bump  
bc96a Merge branch master  
a3b5a Bump again
```

First commit: ‘’Add logic’’. Fair enough.  
Then, “Replace login screen’’. Ok.  
‘’Add button callbacks to new login screen’’. Wait a minute! The previous commit was replacing a working screen by an incomplete one! Possibly it wouldn’t even compile. What good is this previous commit for, then?   
If we were to [git bisect](https://robots.thoughtbot.com/git-bisect) these commits would only fool us.   
I think that the master branch should only have commits with which you would be fairly comfortable to have in prod.   
‘’Add unit tests’’. Was everything untested, then? The previous commits are potentially buggy code?

You see, we’ve put a lot of effort into carefully hand-picking chunks of code and writing commit messages with arguably no benefit. Rarely anyone reviews a PR commit by commit, anyway.

Ideally, we could select chunks of code so that every commit has nice test coverage and the expected behavior. But that takes too much time. Instead, I do the following:

```
bc96a wip  
54be0 code compilable again  
ab4fc fixes
```

When I open the PR I try to write a nice PR description. Finally, I simply squash everything into a single commit and merge it.

{{ medium_image(
    id="1*tFBVxqAGr-yOTX5G3aX5Tw",
    caption="Even though the commit messages aren’t helpful, the PR description should be enough to give the necessary context") }}
    
{{ medium_image(
    id="1*wIsFO0z5cXTo9Te2LABH6g",
    caption="GitHub offers a quick way to squash and merge") }}

It’s time saving and we get a nice history for free, even on terminal!

{{ medium_image(
    id="1*xDRcpV31kRmAkSQgM7WZeg",
    caption="By squashing, we don’t have useless commits") }}

### Shell scripting

Another fundamental skill to be efficient is to write shell scripts hassle-free.We studied programming and are usually hired to develop scalable, high-quality microservices or applications, not for performing tedious, easy tasks.  
If it’s common for you to think that ‘’it would be nice to have a script for X, but the time it’ll take to write it isn’t worth it’’, then let’s think about it as a chemistry reaction:

{{ medium_image(id="1*K00n4FRXv0D7gpSeOOdMBg") }}

If the barrier for writing shell scripts is high, then you’ll stay on the left side, spending high energy to perform tasks.  
However, if you have a catalyst, the equilibrium state will be on the right side. In other words, if it becomes cheaper or easier to write unit scripts, then you’ll spend less energy performing the tasks.

The following practices help me write shell scripts without a hiccup:

Use **set -euo pipefail**. This way you won’t have tricky surprises such as using undefined variables or having failed intermediate commands.

{{ gist(url="https://gist.github.com/denisidoro/fd18de88ee0a3450905af4a3ce2697be") }}

**Namespace your functions**. Let’s understand why:

{{ gist(url="https://gist.github.com/denisidoro/161df2dfeeccc03cde546e24b17bf1a6") }}

What is `is_valid`? We don’t even know where it comes from. If we search for it in the whole project there will be possibly multiple results. Had we namespaced our functions the story would be different.   
If instead it was named as `json::is_valid`, we wouldn’t even need to go to its definition to understand it. But if we wanted to, it would be easy. Also, refactoring without an IDE is easy like this because there’s a single match so a search and replace will safely do the job.  
   
Using [**docopt**](http://docopt.org/) is a must-have, so that you don’t spend time parsing arguments:

<figure>
    {{ gist(url="https://gist.github.com/denisidoro/9ab91e30c9ae277072d1740b710e6d43")}}
    <figcaption>With docopt, you don’t need to write parsers. Instead, you just write the script documentation</figcaption>
</figure>

**Don’t throw away your scripts**, even if they are simple. Instead, put them in a folder, document them and make them available to your disposal whenever you need. I never remember the command for spawning a new Android emulator, for example, but I can always remember how to type `dot android emu start`. In particular, I divide all my scripts into non-nested folders and make them callable by a `dot` command.

```bash
$ $ANDROID_HOME/tools/emulator start     # is it like this?  
$ $ANDROID_SDK/tools/emulator -avd nexus # maybe like this?  
# checks StackOverflow, other references or simply...  
$ dot android emu start                  # there you go!
```

Finally, my biggest tip for bash is… **don’t use bash**.  
Use a language you feel comfortable with or that makes sense to your project.  
Use Python, Ruby or NodeJS.

### Misc

Use a **window manager**. Seriously, the time saved by not resizing, moving, minimizing and maximizing windows is huge. Let’s see a simple demonstration of what it can do:

<figure>
    {{ youtube(id="U17CLayt_aA", class="youtube") }}
    <figcaption>A demonstration showcasing bspwm</figcaption>
</figure>

For Linux, [i3wm](https://github.com/i3/i3) and [bspwm](https://github.com/baskerville/bspwm) are some options; for OSX, [chunkwm](https://github.com/koekeishiya/chunkwm) and spectacle; for Windows, [bugn](https://github.com/fuhsjr00/bug.n).

Have a [**dotfiles repository**](https://github.com/denisidoro/dotfiles) on Github. This way, it’s easy to share your setup with your colleagues. But most importantly: all your setup becomes centralized and you’re always two commands away (or some minutes away) from having your ideal config:

```bash
git clone https://github.com/denisidoro/dotfiles.git ~/.dotfiles  
bash ~/.dotfiles/scripts/environment/init
```

Use **consistent keybindings** across applications. My memory is limited, so it’s nice to memorize commands only once and extend them to other usages. `HJKL`, for example, is reserved for directions in my setup: cursor position and buffer selection on text editors, window selection and resizing on my window manager, scrolling on Chrome, moving between folders on ranger, etc. By the way, **sequential keybindings** help a lot. I can never remember commands like `cmd + option + shift + whatever`, but commands that follow a pattern are pretty straightforward:

```
Focus window left:   CapsLock + H  
Focus window right:  CapsLock + L  
Resize window right: CapsLock + R, L  
Swap window right:   CapsLock + S, L  
Warp window right:   CapsLock + W, L
```

Navigate through your shell **history with ease.** This becomes trivial with fzf:

{{ medium_image(
    id="1*NE-Mw0yyHJ19EmEq1UeAjg",
    caption="fzf gives Ctrl+R steroids") }}

Have a **clipboard manager**. Have you ever tried to paste some text just to realize that you overrode the clipboard with something else then you have to go back to the original application to copy it again? Well, if a clipboard manager this problem is past!

{{ medium_image(
    id="1*M4FsZ1eGCrIABCeusSY9lA",
    caption="With a clipboard manager you have at your fingertips all your recently copied content") }}

**Master** whatever tools you use: terminal emulators, text editors, IDEs, OS, etc.

### Sum up

Of course all I talked about is low level stuff which will save you some minutes or at most a couple of hours at the end of the week. The highest benefits come from high level changes, which may save you entire days of work.

That’s very subjective, though. Maybe what saves you time the most is doing more whiteboards, or avoiding overgeneralizing code, or not checking your emails every single minute. It may even be purely soft skill-related, such as communicating beforehand to your peers what you’re doing. So keep an eye on this kind of optimization as well.

But what I really recommend is… **excel at being lazy**!  
If you’re lazy, you’ll get uncomfortable with time-demanding processes and it will become natural to optimize your workflow.

{{ medium_first(id="save-15min-a-day-tips-for-faster-development-67a31f3498bf" )}}
