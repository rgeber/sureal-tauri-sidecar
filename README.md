# Use SurrealDB with Tauri (WIP)

This example repo shows how to use SurrealDB as a sidecar application with Tauri.

> WARNING: This example will not work out of the box. The required binaries are not included. Follow the instructions
> below to get the binaries.

Tested on Linux with SurrealDB 1.0.0.

By default SurrealDB will be exposed on `http://127.0.0.1:8877`.

## Get the binaries

To keep this clean, small and safe no binaries are included in this repo. Follow these stops to obtain them and put them
in the right location.

First create the bin directory:

```bash
mkdir <PROJECT_ROOT>/src-tauri/bin
```

Download SurrealDB from the [official release page](https://github.com/surrealdb/surrealdb/releases). For Linux on an
amd64 architecture that would be `surreal-v1.0.0.linux-amd64.tgz`.

Unpack to `<PROJECT_ROOT>/src-tauri/bin` and rename `surreal` to `surreal-x86_64-unknown-linux-gnu`.

> The correct file name may vary on your system. Run `rustc -Vv | grep host | cut -f2 -d' '` to get the correct suffix
> for your system. [More information here](https://tauri.app/v1/guides/building/sidecar/)

If you intend to add more platforms just add the correct binaries for each platform with the suffix of choice.

Once the binaries are in place you should be good to go.

## Run the project

> Make sure that all binaries are in `<PROJECT_ROOT>/src-tauri/bin` with the correct suffixes.

First install npm dependencies:

```bash
npm install
```

After that you are good to go to run:

```bash
npm run tauri dev
```

Alternatively you can switch to the `src-tauri` directory and run with `cargo` instead:

```bash
cargo tauri dev
```

## Sidecar vs. Embedded

SurrealDB can be used in an embedded mode witch works great with Tauri. It may be preferrable to use the database
in server mode. That way both the back end and the frond end have client access to the database using their
native client drivers, and you won't need to worry about any kind of IPC channels.

It's a good consideration and also demonstrates the powerful option of bundling any binary you like with your Tauri app.

Using side cars might work great on the Linux desktop but it may be a bad choice for windows or Mac OS due to firewall
restrictions. In this case we open port `8877` on localhost which might trigger some pop-ups.

Another consideration is the use of sidecar binaries on mobile platforms once Tauri supports them. It remains to be
seen if this is a viable option in those environments.

## Other Resources:

* Example repo on using [SurrealDB in embedded mode with Tauri IPC](https://github.com/reymom/surrealdb-starter-taurikit)

## Roadmap & TODO:

* A more visual demonstration on the front end (console.log only for now)
* Using SurrealDB from both back end and front end.
* CLI options for tauri
* Add script to auto-add binaries
* Test on Windows and Mac OS (volunteers with Macs needed :) )
* Optional: Persistent password storage using OS keyring

## Known issues

### Unable to access the database after changing the password

Once SurrealDB is created with a password you may need to stick to that password. If you forgot the password just delete
the database: `rm -rf /tmp/test.db`

### App crashes on reload due to `LOCK` file

SurrealDB is configured to use persistent storage located in `/tmp/test.db`. When npm or cargo reload after a code
change it may be too quick and the lock file is still present.

In a real world scenario I'd recommend to re-try a few times in that case. But for the sake of simplicity I didn't
include it in the example.
