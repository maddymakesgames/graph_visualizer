# Graph Visualizer
This is a Rust program to help visualize various forms of graphs and traversals through them.

## Compiling
For some reason winit requires fontconfig to be installed on some linux distros<br>
On ubuntu / other debian platforms this can be done with:<br>
`sudo apt install libfontconfig-dev`

Otherwise you can just compile using cargo which you can install from the [rust website](https://www.rust-lang.org/learn/get-started).

Precompiled binaries can be found in the github releases
## How to use
To get started you need to create a graph.
This can be done either by typing in a name for the graph and creating an empty graph, or by using the random graph generator to make a random graph.

After that you can use the `nodes` tab to edit the connections between nodes and the `traversal` tab to traverse the graph using various algorithms.

You can also use the `painter settings` tab to edit how the graph is rendered. You can use the `ui settings` tab to edit the font sizes of the ui.

## Current Features
- In-App tutorials 
    - Make the app less cryptic and easier to learn
- Graph Visualization
    - Supports simple graphs, directed graphs, and weighted graphs
- Complete Customization of Visuals
    - Change how any part of the visualization looks
- Executing and Visualizing Traversals
    - Supports Breadth First, Depth First, Dijkstra's Shortest Path, and A*
- Easy(-ish) Graph Creation
    - Can generate random graphs of any kind

## Planned Features
- Easier Graph Creation
    - Support for generating different common forms of graphs
    - Support for parsing text of node connections into a graph
    - Hotkeys to add/remove connections between nodes
- Support for more types of graphs
    - Support for Finite State Machines is a goal I would like to reach
- Even more customization
    - Add more ability to customize the ui
    - Themes? idk!
- Smart Edge / Node positioning?
    - Making it so a bunch of edges don't intersect when they don't have to would be nice!

## Algorithms
The current algorithms used are
- [Breadth-First Search](https://en.wikipedia.org/wiki/Breadth-first_search)
- [Depth-First Search](https://en.wikipedia.org/wiki/Depth-first_search)
- [Dijkstra's Shortest Path](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
- [A*](https://en.wikipedia.org/wiki/A*_search_algorithm)

These are all implemented accurately to my knowledge but if there are inaccuracies please let me know in the issues tab!

If you would like an additional algorithm implemented please open a request in the issues board containing the name of the algorithm and some reference that describes the algorithm in detail.


## Known Bugs
Currently none! Though as with any software there almost surely are bugs, so if you find any reporting them to our issues page would be appreciated!

## License
This project is licensed by the MIT license, and I don't really care what you do with this code.

That said if you redistribute this I would appreciate if you linked back to this repository.