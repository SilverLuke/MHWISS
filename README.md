# MHWISS
Monster Hunter World: Iceborn® Set Searcher is a program for searching the best equipment for your favorite skills.
Written in [rust](https://www.rust-lang.org/) for performance and fun.

Data is provided by [MHWorldData](https://github.com/gatheringhallstudios/).

Current state very simple searcher, do not support decorations.

## Run
Building instruction:
```shell
git clone https://code.elinvention.ovh/SilverLuke/MHWISS.git
cd MHWISS
git submodule init
cd MHWorldData
```
Follow the build instruction [here](https://github.com/gatheringhallstudios/MHWorldData#how-to-build) for building the database.
After that
```shell
cargo run
```

### Images
All the images are provided by [MHWorldDatabase](https://github.com/gatheringhallstudios/MHWorldDatabase) after some processing from android format to standard svg.

### TODO
Add some screenshot

