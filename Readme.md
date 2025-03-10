# ONE MILLION CHECKBOXES (spacetimedb)
This is my attempt at implementing one million checkboxes in timescale db. 

## Get started
This project has two parts. One being the server consiting of rust and spacetimedb. The other being the client written in svelte and the client sdk. 

### Prerequisites
Make sure to install the spacetime cli and bun or any other node package manager
- [Spacetime cli](https://spacetimedb.com/install)
- [Bun](https://bun.sh/)

### Server
The first step is to start a standalone server 
```bash
cd server
spacetime start
```

Create a new terminal and publish the model
```bash
spacetime publish --project-path server one-million-checkboxes
```

That is it for setting up the server. Now let's go to the client frontend

### Client
Download dependencies
```bash
cd client
bun i
```

Build for production
```bash
bun run build
```

Start the client
```bash
bun run preview
```


