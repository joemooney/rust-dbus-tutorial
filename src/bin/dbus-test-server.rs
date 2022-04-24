/* This example creates a D-Bus server with the following functionality:
   It registers the "com.example.dbustest" name, creates a "/hello" object path,
   which has an "com.example.dbustest" interface.
   The interface has a "Hello" method (which takes no arguments and returns a string),
   and a "HelloHappened" signal (with a string argument) which is sent every time
   someone calls the "Hello" method.
*/

use std::sync::Arc;
use dbus::blocking::LocalConnection;
use dbus_tree::Factory;
use std::error::Error;
use std::time::Duration;

// session
// bus:  "com.example.dbustest"   // the "well-known name"
//  |
//  +--- object path: /hello
//         |
//         +---- interface: "com.example.dbustest"
//                 |
//                 +--------method: Hello  // when called we return a list of messages
//                 |                   |
//                 |                   +--------signal: HelloHappened  // when method called
//                 |
//                 +--------signal: HelloHappened  // for introspection
//

fn main() -> Result<(), Box<dyn Error>> {
    // Let's start by starting up a connection to the session bus and request a name.
    let c = LocalConnection::new_session()?;
    c.request_name("com.example.dbustest", false, true, false)?;

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();

    // We create the signal first, since we'll need it in both inside the method callback
    // and when creating the tree.
    let signal = Arc::new(f.signal("HelloHappened", ()).sarg::<&str,_>("sender"));
    let signal2 = signal.clone();

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f.tree(()).add(f.object_path("/hello", ()).introspectable().add(

        // We add an interface to the object path...
        f.interface("com.example.dbustest", ()).add_m(

            // ...and a method inside the interface.
            f.method("Hello", (), move |m| {

                // This is the callback that will be called when another peer on the bus calls our method.
                // the callback receives "MethodInfo" struct and can return either an error, or a list of
                // messages to send back.

                // let name1: &str = m.msg.read1()?;
                // can read up to 5 args, else use read_all iterator
                let (name1, name2) : (&str, &str) = m.msg.read2()?;
                let s1 = format!("Hello {}!", name1);
                let s2 = format!("hello {}!", name2);
                let mret = m.msg
                    .method_return()
                    .append1(s1)  // return value1
                    .append1(s2);        // return value3
                println!("we were called!");

                let sig = signal
                    .msg(m.path.get_name(), m.iface.get_name())
                    .append1(&*name1)   // return value1
                    .append1(&*name2);         // return value2

                // Two messages will be returned - one is the method return (and should always be there),
                // and in our case we also have a signal we want to send at the same time.
                Ok(vec!(mret, sig))

            // Our method has one output argument and one input argument.
            })
            // at run time these return args are checked for being the correct type
            // we may return more than the user expects without error
            // but the client will error if we return fewer
            .outarg::<&str, _>("reply1") // maybe this is used for introspection?
            .outarg::<&str, _>("reply2") // maybe this is used for introspection?
            // at run time these input args are checked for being the correct type
            .inarg::<&str,_>("name1")
            .inarg::<&str,_>("name2")

        // We also add the signal to the interface. This is mainly for introspection.
        ).add_s(signal2)

    // Also add the root path, to help introspection from debugging tools.
    )).add(f.object_path("/", ()).introspectable());

    // We add the tree to the connection so that incoming method calls will be handled.
    tree.start_receive(&c);

    // Serve clients forever.
    loop { c.process(Duration::from_millis(1000))?; }
}
