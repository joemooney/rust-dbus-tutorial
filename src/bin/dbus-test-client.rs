use dbus::blocking::Connection;
use std::time::Duration;

fn remote_call() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_session()?;

    // Second, create a wrapper struct around the connection that
    // makes it easy to send method calls to a specific destination
    // and path.
    let dest = "com.example.dbustest";
    let path = "/hello";
    let method = "Hello";
    let interface = dest;
    let timeout = Duration::from_millis(5000);
    let proxy = conn.with_proxy(dest, path, timeout);
    // let args = "foobar".to_owned();
    let args = ("foo", "bar", );

    // Now make the method call.
    // The ListNames method call takes zero input parameters and 
    // one output parameter which is an array of strings.
    // Therefore the input is a zero tuple "()",
    // and the output is a single tuple "(names,)".
    // At runtime we will get an error if we do not get at least
    // this number of return args, we could get more and then
    // will be ignored
    let (name1,name2,): (String,String,) = proxy.method_call(interface, method, args)?;
    // let (name,): (String,) = match proxy.method_call(interface, method, args) {
    //     Ok(v) => { let (name,)  = v; name }
    //     Err(e) => return Err(e.into())
    // };

    // Let's print all the names to stdout.
    println!("<{}> <{}> ", name1, name2);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    remote_call()?;

    Ok(())
}