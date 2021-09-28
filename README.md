# MHWISS
Monster Hunter World: Iceborn® Set Searcher is a program for searching the best equipment for your favorite skills.
Written in [rust](https://www.rust-lang.org/) for fun, learning and (maybe) performance.

Data is provided by [MHWorldData](https://github.com/gatheringhallstudios/).

Current state: very simple and inaccurate searcher.

## Run
Building instruction:
```shell
git clone https://code.elinvention.ovh/SilverLuke/MHWISS.git
cd MHWISS
git submodule update --init
cargo run
```

While the pc is compiling you should build the database, go in MHWorldData directory (`cd MHWorldData`) and follow the build instruction [here](https://github.com/gatheringhallstudios/MHWorldData#how-to-build).

### Create your own engine
You want implement your own engine for searching the bleeding edge equipment in MHW? You can do it with some simple steps:

- Add your engine name in the enum Engines inside the ``src/engines.rs`` file.
- Add a match case inside the match in the run method in `src/engines.rs` file.
- Create a new file in `src/engines` directory with our engine.

## Images
All the images are provided by [MHWorldDatabase](https://github.com/gatheringhallstudios/MHWorldDatabase) after some processing from android format to standard svg.

### TODO
Add some screenshot

