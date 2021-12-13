+++
title = "Faster monorepo workflow with materialized views"
[taxonomies]
tags = [ "git", "monorepo" ]
[extra]
summary = "An approach for faster local development of monorepos"
+++

### Context

Monorepos have their pros and cons. 

A plethora of in-depth articles have already been written on this subject so I won't bother writing yet another one. [This one][comparison] summarizes the trade-offs very well.

In this post, I propose a solution for improving the dev experience for monorepos, by using microrepos as materialized views for subprojects and a bot as orchestrator.

A [proof-of-concept CLI][cli] for managing repositories is also provided, which can be used as base for a real-world tool set. 

## Assumptions

To keep this post concise, I'll list some assumptions under which this solution was designed. 

Depending on your workflow or the requirements of the code base you're developing, alternative solutions should be considered. 

1. git must be used, as changing the SVN is too disruptive
2. the main drawback of monorepos are slow git operations in dev machines
3. if git were performant for large repos, monorepos would clearly be superior to microrepos, with few, if any, downsides 

## Proposed solution 

<figure style="margin-left: 0; margin-right: 0">
    <img src="/posts/git-monorepo-architecture.png">
    <figcaption style="font-size: 0.8em">In a nutshell, devs work with materialized views an the bot propagates changes</figcaption>
</figure> 

### Repo setup

- 0.a. Create a monorepo `mono` as the source of truth of our codebase. It should include all subprojects. Let's say it contains `proj1` and `proj2`
- 0.b. Create microrepos for `proj1` and `proj2`. They'll act as materialized views
- 0.c. Protect the `master` branch of all microrepos. Only the bot should be able to commit to `master`

### Workflow

1. Instead of cloning `mono`, the dev should clone microrepos of interest. Let's say only `proj2`
2. The dev creates a branch `newfeature` in `proj2`
3. After making the necessary changes, the dev pushes this branch to `proj2`'s remote, not `mono`'s
4. A bot creates a PR in `mono`, reflecting the changes of `proj2`'s `newfeature`
5. The dev hits the "merge pull request" button for the PR in `mono`
6. For each commit to `mono`'s `master`, the bot commits to the microrepos' `master` accordingly. In this case, `proj2`'s master will eventually include the changes from `newfeature`

## Making changes to multiple repos at once

The bot that creates the PR in `mono` must be able to aggregate related branches from multiple microrepos.

In order for the bot to know if two branches are related or not, an identifier can be used. For example, if `newfeature` was used as a branch name in `proj1` and the feature requires changes to both projects, a namesake branch in `proj2` must be created.

## Why this solution is good

In a nutshell, because the advantages of monorepos are kept. The only practical difference is that devs don't need to run slow git operations on their machines. 

The good news is that all this can be abstracted away by a CLI. 

## Does this solution need to be so complex?

I think so. 

Multiple multi-billion dollar companies struggle with this problem. If there were a simple solution, I'm sure someone would already have figured it out.

The only simple solution (from end-user's perspective) I can think of is to have an SVN performatic for monorepos out-of-the-box. 

Perhaps that's the case already, but a different SVN rejects our assumption `1.`.

## Rejected solution

### Having the microrepos as source of truth and the monorepo as materialized view

The problem with this solution is that changes to `proj1` and `proj2` must be atomic, assuming a feature requires changes to both: we either want to commit a change to both projects or drop the commit altogether. 

git currently doesn't provide a solution for such transactions, so a mechanism for simulating atomicity would need to be designed, rendering the solution even more complex. 

For example, a change to `proj1` would need to be reverted in case we're not able to commit to `proj2`. 

As we all know, distributed systems can fail or become inconsistent for all sorts of reasons. If somehow `proj2` got corrupted or inconsistent, it's much easier and less error-prone to fix or reconstruct its materialized view than trying to agree upon the source of truth.

## Demo

To illustrate, I could've [created a GitHub bot][probot]. That would exceed the time budget I set for putting this article together, though. 

For demo purposes I've created a [proof-of-concept CLI][cli] that simulates the flow locally. This won't simulate the interactions with PRs, as they don't exist in a local machine, but will give us a clear idea of how this flow works. 

In this example, all folders inside `~/github` represent repositories you would normally have hosted on GitHub; all folders inside `~/dev` represent the local clones. 

Once this CLI is available in your `$PATH` as git-monorepo, you can invoke it by running `git monorepo`. 

You can execute the commands below in your local machine if you want to follow along. The CLI prints all commands it's running for you to understand what's happening under the hood. 

Without further ado, let's get to it.

### 0.a. Setting up the monorepo

Let's create the remote `mono` repository:

```sh
mkdir -p ~/github/mono
cd ~/github/mono
git init
mkdir proj{1,2}
for i in 1 2; do echo "console.log('proj${i}')" > proj${i}/file${i}.js; done
git add . 
git commit -am 'First commit' 
```

By the end of these steps, GitHub would host a monorepo like this:
```sh
mono/
   proj1/file1.js
   proj2/file2.js
```

### 0.b. Setting up the microrepos

Let's create the remote microrepos:

```sh
for i in 1 2; do git monorepo extract proj${i} ~/github/proj${i}; done
```

The first argument is the path to the project inside `mono`; the second argument is where the remote microrepo will live.

Normally, the second argument would look like `https://github.com/username/proj1` or `git@github.com:username/proj1`

By the end of these steps, GitHub would host repositories like this:
```
mono/
   .gitmonorepo
   proj1/file1.js
   proj2/file2.js
proj1/
   file1.js
proj2/
   file2.js
 ```
 
The `.gitmonorepo` was automatically created to keep track of the microrepos. 

### 1. Cloning a microrepo

Let's clone `proj1`:

```sh
mkdir -p ~/dev
cd ~/dev
git clone -b master ~/github/proj1
```

### 2. Making changes

Let's develop a new feature. 

```
cd ~/dev/proj1
git checkout master
git pull origin master
git checkout -b newfeature
echo "console.log('newchange')" >> file1.js
git add .
git commit -am "proj1/newfeature: change file1.js"
```

### 3. Pushing a change to the microrepo

Let's push our changes to the remote `proj1`:
```sh
git push origin newfeature
```

### 4. Propagating the changes to the monorepo

The bot would automatically propagate the changes to `mono`, by running something like the following:

```sh
cd ~/github/mono
git monorepo pull newfeature
```

Now, `~/github/mono/proj1/file1.js` should have the `newchange` line on the `newfeature` branch, but not in `master`. 

### 5. Merging a PR

Let's merge our branch:

```sh
git checkout master 
git merge newfeature
```

Now, `~/github/mono/proj1/file1.js` should have the `newchange` line on `master`. 

### 6. Propagating the change back to the microrepo

Finally, the bot would automatically update all microrepos accordingly, by running something like the following:

```sh
git monorepo push
```

Now, `~/dev/proj1/file1.js` should also have the `newchange` line on `master`, ending the loop cycle. 

Please note that we were able to make changes to the remote monorepo having only cloned `proj1`. `proj2` and `mono` weren't cloned locally.

## Future work

We've only covered the simple, happy path so far. 

Ideally, this system should also include:
- a dev-friendly git wrapper for working with multiple microrepos at once
- a UI for displaying how the microrepos and the monorepo are interacting with each other 
- different resolution strategies, in case one of the propagation changes fails for some reason
- merge queues
- a mechanism for replicating the monorepo locally, but using the microrepos of interest instead 
- cleanup routines, for deleting branches in microrepos whose PR in the monorepo is closed 
- fixes to [these TODOs][todo]
- and much more. 

## Conclusion

The purpose of this article was to simply brainstorm what a more performant workflow could look like. 

I hope that this will motivate someone out there willing to implement a system ready for real-world scenarios. 

In case you do, I'd really appreciate if you could add a link to this post somewhere in your README.md file! :) 

[comparison]: https://github.com/joelparkerhenderson/monorepo-vs-polyrepo
[cli]: https://github.com/denisidoro/git-monorepo
[probot]: https://github.com/probot/probot
[todo]: https://github.com/denisidoro/git-monorepo/search?q=TODO