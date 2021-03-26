
/*

Current design,

Apps:

	+ Apps are the core pattern; they are built by developers and "do some work" of some kind

	+ Apps have a single function method call as their entry point, but they can spawn off a thread

	+ Apps are handed an outbound message channel which lets them talk to a broker that can route traffic

	+ Apps are also handed an inbound message channel for any messages they should process

Pubsub:

	+ Apps can register to listen to any public message being sent to the broker in general

	! Right now messages are copied; later we want some kind of shared memory messaging

	- Later apps should be able to listen specifically to each other more tightly

	- Later apps should be able to know what other apps exist and also express dependencies

	- Later (the main point) apps can load up WASM blobs to basically drive work via scripting

	- Later the messaging will be a way to enforce security policy between apps

Overall:

	- I am imagining a 'sea' of "medium sized agents" or "lighterweight apps" or "modules" or "code"
	- These are not full blown processes isolated by the real operating system kernel or MMU
	- These are isolated from each other by rust and wasm paradigms; so - basically "softer" walls
	- I see some of these agents listening to events "in general" similar to javascript DOM listeners
	- I see some of these agents requiring other agents to exist and then connecting to them
	- I see security being imposed by controlling the connections
	- I don't see any other way for agents to communicate; I prefer to control the comms myself

*/

use appbroker::*;

mod camera;
mod face;
mod display;

fn main() {

	let mut broker = AppBroker::new();

	broker.add("Camera",camera::app);
	broker.add("Face",face::app);
	broker.add("Display",display::app);

	broker.run();

}

