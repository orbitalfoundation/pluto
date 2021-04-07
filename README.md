# Prototype

## What is this?

This is a rough cut throwaway sketch of a userland web-app-runner. It is exploring an idea of running wasm based applications that are fetched over the wire. Conceptually it could be considered similar to a Desktop or Steam or any kind of modern app manager.

This is intended to act like a new kind of web browser in that users can visit URLS, load and show the contents of that URL and generally have a similar kind of experience conceptually to the traditional web.

The difference is that this browser also acts a bit more like Steam, or an App Store in that it's focused around an idea of downloading persistent or durable applications that it then helps the user manage. In fact the core system doesn't even natively support HTML layout at all. The plan is to only support WASM blobs and nothing else. There is an expectation that applications will be able to have a list of dependencies on other WASM modules; not dissimilar from ordinary package managers that we're used to such as NPM, Crates or WAPM.

This is all super early, the code is rough, very fragile, missing key features. It's not usable for any kind of real world purpose yet. It's purely a design-in-code sketch at the moment. The plan is to keep iterating on this core however.

## What are we specifically exercising in this prototype?

### Product Core:

Some thought has been putt into a concept of structuring units-of-computation as a set of peers in a microkernel or microservice architecture. There are these pieces in the core implementation:

1. Services. There is a concept of a 'service'. A service is a self contained 'unit of computation'. There are formally two kinds of services:

	Rust Services : There are built-in or hard-coded services which implement raw/unsafe access to devices (camera inputs, display outputs).
	WASM Services : WASM services load up a WASM blob from any remote source, on the fly, at runtime. These are "user apps".

2. Threads. Services are separate pre-emptive threads. It's important to us to have pre-emptive threading. WASM services can use this as well.

3. Messages. Services may message each other through crossbeam channels across threads.

4. Broker. Right now there is a special discovery service that brokers messages. It implements only pub sub for now (no shared memory messages yet).

### Display Service Specifically:

Although display is not "core" it is so important that also some thought has been put into this as well:

5. UX. We have a minimal hypervisor UX that shows a privileged or unblockable command line / input box to lets users type in an URL and load up that URL.

6. Display Module. We exercise Makepad's new display solution which is GPU focused and highly performant as a way to deliver the user experience overall.

### Example / Test Service:

Attached to this (in the same source tree) is an example of how we imagine this service may be used. Here is what it does:

1. Loads a WASM blob dynamically. We show that this "browser" can late-load applications over the wire.

2. WASM blobs can do useful work. We show that late loaded applications can drive the display and other "built in" services.

3. Usefulness; we wanted to show some kind of useful example; in this case we examine camera frames and segment faces and display that segmentation.

### Areas to improve:

There's some insight we've had already, and here are some of the areas for improvement ( note that this entire code base will likely be thrown away but some of the patterns will remain):

1. Messaging is fairly simple right now. Later services should be able to directly bind to each other, including having shared memory.

2. Standards; right now all device access (camera inputs, displays) are all completely custom; we'd want to standardize on conventions (WebXR?).

3. Graphics; we want to dramatically improve graphics rendering and output to have a vastly more capable visualization experience for users.

4. UX; we want to improve the built in UX to include a desktop, an enumeration of all apps and services and a management panel overall.


### Noted transient bugs / issues to look at

- i think i need some kind of display abstraction / scenegraph? or some kind of hashed list of what is painted to a view
- general: may make sense to use more generic methods from a trait such as "paint()" rather than paint_a_thing_that_is_an_input_button()
- textwidget: emoji?
- textwidget: detecting carriage return -> returned events could be richer; also; why one event per call? are these stacked up?
- textwidget: text is backwards sometimes
- textwidget: width? -> dynamic width is a hassle
- linking: packaging dependencies as separate crates better such as makepad itself -> needs a new version probably

### TODO -> stories

	* wasm story 1;
		* order camera to yield frames
		* face recognizer automatically segment faces
		- real camera, get real frames
		- real face segmenter (maybe skip this)
		- figure out how to pass an image to display
		- wasm blob not stop

	* wasm story 2
		* just paint some cubes
		x later a weather app? a navigation app? friend finder? try focus on apps for groups not for individuals

	- display story
		* let users actually load apps; input box and so on
		* let apps order the display to paint something
		- each app should hash or identify a handle on what it owns so display can show it or not as it wishes
		- paint a list of apps both live and stashed on a side bar
			- broker or somebody needs to be able to report a list of existing apps
			- broker should actually persist apps as well; not just pretend to
			- display needs to be able to paint a list of running apps
			- running apps should be clickable and there should be state on them; permissions and so on
			- also need to be able to switch between apps; give certain apps focus


