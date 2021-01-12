+++
title = "Writing multi-platform mobile apps with React Native and KotlinJS"
[taxonomies]
tags = [ "dev", "kotlin", "js", "react" ]
[extra]
summary = "Kotlin is an awesome language. React Native is becoming more and more popular. Why not mixing both?"
+++

{{ medium_cover(id="0*EnXaEGp3rgRz7NS4.") }}

Kotlin is an awesome language. It is functional and offers [extension functions](https://kotlinlang.org/docs/reference/extensions.html), [null safety](https://kotlinlang.org/docs/reference/null-safety.html), [smart casting](https://www.callicoder.com/kotlin-type-checks-smart-casts/), [type-safe builders](https://kotlinlang.org/docs/reference/type-safe-builders.html), [default and named arguments](https://www.callicoder.com/kotlin-functions/), [data classes](https://www.callicoder.com/kotlin-functions/) and much more. It is arguably the language of choice for Android developers and [very familiar](http://nilhcem.com/swift-is-like-kotlin/) to iOS developers.

React Native enables us to write only once for multiple platforms, has a simpler API than native development and a smooth learning curve. Steve Yegg, a former Google engineer, conjectures in [his blog post](https://medium.com/@steve.yegge/who-will-steal-android-from-google-af3622b6252e) that hybrid frameworks are likely to become even more popular. And the framework from Facebook is the major player at the moment.

So why not mixing both?   
Standing on the shoulder of giants ([JetBrains/kotlin-wrappers](https://github.com/JetBrains/kotlin-wrappers/tree/master/kotlin-react) and [ScottPierce/kotlin-react-native](https://github.com/ScottPierce/kotlin-react-native)), we can.   
An example repository can be found [here](https://github.com/denisidoro/korn).

### React in a nutshell

You can learn the basics of React Native very quickly reading the [official tutorial](https://facebook.github.io/react-native/docs/tutorial.html), but three important takeaways are:  
- view = f(state)   
- React has a virtual representation of the view, and it calculates the smallest diff between renders  
- development should be component-driven

### Hello world in Kotlin

After cloning the [example repository](https://github.com/denisidoro/korn) and following the README, you can simply edit the render() function to immediately see the result on the device.

For example, writing the following code…

{{ gist(url="https://gist.github.com/denisidoro/64a189c08b57e50440c7e5bc9bcdc1a2") }}

…would render the following output:

{{ medium_image(
    id="1*QGZmQ_Zq7_jTQXXiBWFILg",
    caption="Our first application with Kotlin and React Native") }}

Thanks to the Kotlin language features, we can write the view using a builder DSL similar to [JSX](https://reactjs.org/docs/jsx-in-depth.html), instead of manually calling React.createElement(…, React.createElement(…)).

### Creating our first component

Let’s say you have the following view architecture and you want to make the code for headers reusable:

{{ gist(url="https://gist.github.com/denisidoro/3264c01ad43aa051893d6937a22aa850") }}

For this task, you could create a Header component:

{{ gist(url="https://gist.github.com/denisidoro/bbe3b31572c52acfecc76f2ecc50bd03") }}

And call it from the render() function as such:

{{ gist(url="https://gist.github.com/denisidoro/bd46d12af1fd4188393bdde48f2c02a1") }}

Alternatively, if you want the extend the DSL so that you can call this component directly, just create an extension function:

{{ gist(url="https://gist.github.com/denisidoro/2ab29ea9159c3f55285bd52ac8e5ac95") }}

### Managing state

React doesn’t dictate how state management should be. For the sake of this example, let’s write our own simplified, naive implementation of [Redux](https://redux.js.org).

In this pattern, we have three main elements:  
- store: holds the state  
- actions: describe the changes  
- reducers: return an updated state based on the requested action

Here is the full implementation:

{{ gist(url="https://gist.github.com/denisidoro/c60df8ce463f92e03d74926ad6bd2328") }}

Evidently, on a real world application, you should prevent race conditions and implement the subscriptions in an atomic fashion, possibly using [RxJS](https://github.com/reactivex/rxjs). But that’s good for now.

### A less contrived example

Now let’s write a simple application which consists of a header, and two rating widgets, for giving scores to two different programming languages.

First we create the models:

{{ gist(url="https://gist.github.com/denisidoro/c9b8b14b9f43500ad243339fe78fb792") }}

Then the actions that can be dispatched:

{{ gist(url="https://gist.github.com/denisidoro/fddebcf58c3b6330213b74868cd8fec8") }}

Later on, we define the reducer:

{{ gist(url="https://gist.github.com/denisidoro/c3c46f8e78e30d476e6ca9b17c46995e") }}

Then we define a view model, mapping state to view abstractions:

{{ gist(url="https://gist.github.com/denisidoro/f7bf0931eaa4acc382386faf804a3fc4") }}

We connect everything with a store:

{{ gist(url="https://gist.github.com/denisidoro/f308d63f310d6f52cfd85e7a0d706263") }}

Finally, we create a stateless, dumb component:

{{ gist(url="https://gist.github.com/denisidoro/bc52bf9cc2c899cde5f0ad4b383190bc") }}

And another component that keeps track of the state:

{{ gist(url="https://gist.github.com/denisidoro/cefc050a3e2e5798c53ddd387d3ae94f") }}

And the final result is:

{{ medium_image(
    id="1*EDiObUxyPdrQpEFEKovdKQ",
    caption="Our final application") }}

### So… Is Kotlin with React Native production-ready?

Possibly, but there are still some improvements to be made to the whole ecosystem. The interoperability with JavaScript isn’t effortless and has some pitfalls. For example, passing the store via props to the component won’t work because the state will become immutable. Also, using data classes for the props directly will return an error from React because apparently the serialization contains some metadata. In addition, you’re supposed to write your own type definitions, which is a huge productivity killer.

I think JetBrains is to blame for the unpopularity of KotlinJS, by not giving the language the love it deserves. [Kotlin/ts2kt](https://github.com/Kotlin/ts2kt), for instance, could solve the type definition problem. However, most of the attempts for converting types fail, and the [issues remain open](https://github.com/Kotlin/ts2kt/issues). Actually, we don’t see many examples targeting JS at all, despite the fact that this a working platform for more than a year. Maybe they are focused on [Kotlin/Native](https://kotlinlang.org/docs/reference/native-overview.html)…

In the meantime, another alternative to JavaScript is [Clojurescript](https://clojurescript.org/), which is awesome with [re-frame](https://github.com/Day8/re-frame) and in my opinion offers the best development experience out there. However, it’s not for the faint-hearted.

I hope that with this article and the example repository, the community can reconsider Kotlin as a language for React Native. Feel free to [fork the code](https://github.com/denisidoro/korn)!

{{ medium_first(id="writing-multi-platform-mobile-apps-with-react-native-and-kotlin-50f486b53462") }}
