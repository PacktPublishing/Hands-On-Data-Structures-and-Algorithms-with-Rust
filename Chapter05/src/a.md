# B Trees




## A Device Database

## 

## A Device Database

# Graphs

In their most generic form, trees are graphs - directed, acyclic graphs. A general graph can be described as a collection of connected nodes, sometimes referred to as vertices, with certain properties like whether cycles are allowed. The connections between those also have their own name: edges. These edges can have certain properties as well, in particular weights and directions (like one-way streets).

By enforcing these constraints, a model can be built that - just like trees - reflect a certain reality very well. There is one particular thing that is typically represented as a weighted graph: the Internet. While nowadays this might be an oversimplification with various versions of the internet protocol (IPv4 and IPv6) and network address translation technologies (NAT) hiding large parts of participants online. In its earlier days however, the internet can be shown as a collection of routers, computers, and servers (nodes) interconnected with links (edges) defined by speed and latency (weights).

!# Image: Internet

Other than humans, who can typically see and follow a reasonably efficient path through this mesh of interconnected nodes, computers require specific instructions to find anything in there! This called for new algorithms that allow for dealing with this complexity - which is especially tricky once the number of nodes in the mesh exceeds the number of nodes that can be looked at in time. This led to the development of many routing algorithms, techniques to finding cycles and segmenting the network, or popular NP-hard problems like the traveling salesman problem or the graph-coloring problem. 

!# image: TSP 

Today there are many examples of graphs, the most obvious being a social graph (in social networks), but also as part of Tensorflow's deep learning API, state machines, and the rise of graph databases that offer a generic query language to traverse graphs. Even some less obvious use cases can be found like storing genetic sequences (nodes being the small parts of the DNA)! 

To get out of theoretical constructs, how would you represent a graph in a program _efficiently_? As a node struct with a list of outbound vertices? How would you find a particular node then? A tricky problem! Graphs also have the habit of growing quite large, as anyone who ever wanted to serialize object graphs to JSON can testify: they run out of memory quite easily. 

The best way to work with this data structure is surprisingly simple: a matrix. This matrix can either be sparse (i.e. a list of lists with varying sizes), called an adjacency list, or full-blown matrix (adjacency matrix). Especially for a matrix, the size is typically the number of nodes on either side and the weights (or boolean values representing "connected or not") at each crossing. Many implementations will also keep the "real" nodes in its own list, using the indices as ids.

!# image: matrix vs graph

Rust provides many great tools for implementing really complex graph structures: enums and pattern matching provide ways to operate on types of nodes and edges with low overhead, iterators and functional approaches remove the need for verbose loops. Let's look at a generic graph struct in Rust:

!# code: simple Graph structure in Rust, adjacency list

This adjacency list can store nodes and whether they are connected, making this a finite, indirected, unweighted graph - great for storing simple relationships between objects. Already, a data structure like this has the ability to implement sophisticated routing algorithms or run out of resources on a backtracking algorithm. In an adjacency list, each index in the list represents the origin of an edge and the contained elements (also lists) are any outbound edges. To traverse the graph, start at an origin index and find the next index by searching its edges. Then repeat until arriving at the destination node!

When the product team has heard of this amazing data structure - and they are now well aware of your abilities - they came up with a new product: the literal Internet Of Things (it's a working title). Their idea is to provide customers with a way to model complex sensor placements that would have distance built in! Customers can then go and evaluate all sensors that are within a certain range of each other, find single points of failure, or plan a route to inspect them quickly. 

To summarize, customers should be able to:

- Create add a list of nodes
- Connect nodes with their physical distance to each other
- Find the shortest path between two nodes with respect to the distance provided
- Retrieve a list of neighbors of a specified node, up to a certain degree

Great idea, right? A great fit for graphs as well. 


## The Literal Internet Of Things

In order to get a head start on these requirements, the decision for a graph representation has to be made: list or matrix? Both work well, but for explanatory reasons the examples will go with an adjacency list built on top of a vector of vectors:

~~~
pub struct InternetOfThings {
    adjacency_list: Vec<Vec<Edge>>,
    nodes: Vec<KeyType>,
}
~~~

As previously mentioned, it makes sense to keep the actual values, identifiers, or even entire objects in their own list and simply work with indices of type usize. The Edge struct in this example could be just represented as a tuple just as well, but it's way more readable this way:

~~~
#[derive(Clone, Debug)]
struct Edge {
    weight: u32,
    node: usize,
}
~~~

Having those two structures in place, adding nodes (or ... things) to the graph can be done with only a few lines:

~~~

fn get_node_index(&self, node: KeyType) -> Option<usize> {
    self.nodes.iter().position(|n| n == &node)
}

pub fn set_edges(&mut self, from: KeyType, edges: Vec<(u32, KeyType)>) {
    let edges: Vec<Edge> = edges.into_iter().filter_map(|e| {
        if let Some(to) = self.get_node_index(e.1) {
            Some(Edge { weight: e.0, node: to }) 
            } else {
                None
            }}).collect();
    match self.nodes.iter().position(|n| n == &from) {
        Some(i) => self.adjacency_list[i] = edges,
        None => {
            self.nodes.push(from);
            self.adjacency_list.push(edges)
        }
    }
}
~~~

Within that function, there is a crucial check that's made: every edge has to connect to a valid node, otherwise it will not be added to the graph. To achieve this, the code looks up the ids provided in the edges parameter in its internal node storage to find the index it's at - something that is done by the position() function of Rust's Iterator trait. It returns the position of when the provided predicate returns true! Similarly, the filter_map() function of the iterator will only include elements that evaluate to Some() (as opposed to None) in its result set. Therefore the nodes have to have a setter that also initializes the adjacency list:

~~~
pub fn set_nodes(&mut self, nodes: Vec<KeyType>) {
    self.nodes = nodes;
    self.adjacency_list = vec![vec![]; self.nodes.len()]
}
~~~

Once that's done, the graph is ready to use. How about we go looking for neighbors first?



## Neigborhood Search

Neighborhood search is a very trivial algorithm: starting from the provided node, follow every edge and return what you find. In our case, the degree of the relationship is important, but it could just as well be the added weights of that are a cut-off for the search. 

Just like for the previously shown tree algorithms, recursion is a great choice for solving this problem. While an iterative solution will often be more memory efficient (no stack overflows), recursion is way more descriptive once you get the hang of it. Additionally, some compilers (and partly rustc, but not guaranteed) will expand the recursion into a loop - providing the best of both worlds (look for tail call optimization)! Obviously, the most important thing is to have a projected growth in mind, 10 0000 recursive calls are likely to fill up the stack.

However, the function to run the neighborhood is implemented two-fold. First the public-facing function takes care of validating input data and sees if the node actually exists. 

~~~
pub fn connected(&self, from: KeyType, degree: usize) -> Option<HashSet<KeyType>> {
    self.nodes.iter().position(|n| n == &from).map(|i| {
        self.connected_r(i, degree).into_iter().map(|n| self.nodes[n].clone()).collect()
    })
}
~~~

With that out of the way, the recursive call can create a list of all its neighbors and run the same call on each of them. Returning the resulting nodes in a set eliminates duplicates as well. 

~~~
fn connected_r(&self, from: usize, degree: usize) -> HashSet<usize> {
    if degree > 0 {
        self.adjacency_list[from]
            .iter()
            .flat_map(|e| {
                let mut set = self.connected_r(e.node, degree - 1);
                set.insert(e.node);
                set
            }).collect()
    } else {
        HashSet::new()
    }
}
~~~

Since the recursive call returns the internal representation (i.e. indices), the outer function translates those back into data the user can understand. This function can serve as the basis of other features like intersecting the neighborhoods of two nodes, vicinity search, etc. Or to make it more real: on a sensor outage, the company can check if there is a common device that's responsible (intersection), or if other close-by sensors are reporting similar measurements to rule out malfunctions (neighborhood search). Let's move on to something more complex: finding the shortest path.

## The Shortest Path

This algorithm has its roots in early networking: routers had to decide where to forward to packets to, without having any knowledge of what's beyond. They simply had to make the best decision without having perfect information! Edsger Dijkstra, one of the pioneers of computer science, then came up with a graph routing algorithm that has been named after him: Dijkstra's algorithm. 

The algorithm works iteratively and goes over each node to add up their weights, thereby finding the distance (or cost) of reaching this node. It will then continue at the node with the lowest cost, which makes this algorithm a "greedy" algorithm. This continues until the desired node is reached or there are no more nodes to evaluate.

!#> info box greedy algorithms vs local optima.

In code, this is what that looks like:
~~~
pub fn shortest_path(&self, from: KeyType, to: KeyType) -> Option<(u32, Vec<KeyType>)> {
    let mut src = None;
    let mut dest = None;

    for (i, n) in self.nodes.iter().enumerate() {
        if n == &from {
            src = Some(i);
        }
        if n == &to {
            dest = Some(i);
        }
        if src.is_some() && dest.is_some() {
            break;
        }
    }
    if src.is_some() && dest.is_some() {
        let (src, dest) = (src.unwrap(), dest.unwrap());

        let mut distance: Vec<TentativeWeight> =
            vec![TentativeWeight::Infinite; self.nodes.len()];
        distance[src] = TentativeWeight::Number(0);

        let mut open: Vec<usize> = (0..self.nodes.len()).into_iter().collect();
        let mut parent = vec![None; self.nodes.len()];
        let mut found = false;
        while !open.is_empty() {
            let u = min_index(&distance, &open);
            let u = open.remove(u);

            if u == dest {
                found = true;
                break;
            }

            let dist = distance[u].clone();

            for e in &self.adjacency_list[u] {
                let new_distance = match dist {
                    TentativeWeight::Number(n) => TentativeWeight::Number(n + e.weight),
                    _ => TentativeWeight::Infinite,
                };
                
                let old_distance = distance[e.node].clone();

                if new_distance < old_distance {
                    distance[e.node] = new_distance;
                    parent[e.node] = Some(u);
                }
            }
        }
        if found {
            let mut path = vec![];
            let mut p = parent[dest].unwrap();
            path.push(self.nodes[dest].clone());
            while p != src {
                path.push(self.nodes[p].clone());
                p = parent[p].unwrap();
            }
            path.push(self.nodes[src].clone());

            path.reverse();
            let cost = match distance[dest] {
                TentativeWeight::Number(n) => n,
                _ => 0,
            };
            Some((cost, path))
        } else {
            None
        }
    } else {
        None
    }
}
~~~

Since this is a long one, let's break it down. This is boiler plate code to ensure both source and destination nodes are nodes in the graph. 

~~~
pub fn shortest_path(&self, from: KeyType, to: KeyType) -> Option<(u32, Vec<KeyType>)> {
    let mut src = None;
    let mut dest = None;

    for (i, n) in self.nodes.iter().enumerate() {
        if n == &from {
            src = Some(i);
        }
        if n == &to {
            dest = Some(i);
        }
        if src.is_some() && dest.is_some() {
            break;
        }
    }
    if src.is_some() && dest.is_some() {
        let (src, dest) = (src.unwrap(), dest.unwrap());

~~~

Then, each node gets a tentative weight assigned, which infinite in the beginning, except for the origin node, which has a zero cost of being reached. The "open" list, which contains all the nodes yet to be processed, is conveniently created using Rust's Range - as it corresponds to the indices we are working with. The parent array keeps track of each node's parent once the lower cost is established, which provides a way to trace back the best possible path!

~~~
        let mut distance: Vec<TentativeWeight> =
            vec![TentativeWeight::Infinite; self.nodes.len()];
        distance[src] = TentativeWeight::Number(0);

        let mut open: Vec<usize> = (0..self.nodes.len()).into_iter().collect();
        let mut parent = vec![None; self.nodes.len()];
        let mut found = false;
~~~

Now, let's plunge into the pathfinding. The helper function min_index() takes the current distances and returns the index of the node that is easiest (as in lowest distance) to reach next. This node will then be removed from the open list. Here's a good point to also stop if the destination has been reached, for more thoughts on this see the info box on greedy algorithms above. Setting found to true, will help distinguish between no result and early stopping.  

For each edge of this node, the new distance is computed and, if lower, inserted into a distance list (as seen from the source node). There are a lot of clones going on as well, which is due to ensure not borrowing while updating the vector. With u64 (or u32) types this should not create a large overhead (pointers are typically that large too), but for other types this can be a performance pitfall. 

~~~
        while !open.is_empty() {
            let u = min_index(&distance, &open);
            let u = open.remove(u);

            if u == dest {
                found = true;
                break;
            }

            let dist = distance[u].clone();

            for e in &self.adjacency_list[u] {
                let new_distance = match dist {
                    TentativeWeight::Number(n) => TentativeWeight::Number(n + e.weight),
                    _ => TentativeWeight::Infinite,
                };
                
                let old_distance = distance[e.node].clone();

                if new_distance < old_distance {
                    distance[e.node] = new_distance;
                    parent[e.node] = Some(u);
                }
            }
        }
~~~

After this loop exits, there is a distance array and a parent array to be prepared for returning to the caller. First, trace back the path from the destination to the origin node in the parents array, which leads to the reverse optimal path between the two nodes. 

~~~
        if found {
            let mut path = vec![];
            let mut p = parent[dest].unwrap();
            path.push(self.nodes[dest].clone());
            while p != src {
                path.push(self.nodes[p].clone());
                p = parent[p].unwrap();
            }
            path.push(self.nodes[src].clone());

            path.reverse();
            let cost = match distance[dest] {
                TentativeWeight::Number(n) => n,
                _ => 0,
            };
            Some((cost, path))
        } else {
            None
        }
    } else {
        None
    }
}

~~~

By strictly following the node with the lowest distance, Dijkstra's algorithm achieves a great runtime when stopping early and runtime can even be improved by using more efficient data structures (like a heap) to fetch the next node efficiently.

Modern approaches to shortest paths in a graph typically use the A* (pronounced "A Star") algorithm. While it operates on the same principles, it is also a bit more complex and would therefore go beyond this book. 


## Wrap Up

A graph is surprisingly easy and straightforward to implement: clear ownership in adjacency lists or matrices makes them almost effortless to work with! On top of that, there are two additional aspects that weren't yet covered in this implementation: an enum with an implementation and using regular operations (here: comparison) with this implementation. 

This shows how conforming to standard interfaces provides great ways to interface with the standard library or well-known operations in addition to the flexibility enums provide. With a few lines of code, infinity can be represented and worked with in a readable way. It was also a step towards more algorithmic aspects, which will be covered later in the book. For now, let's focus on graphs again. 


### Upsides

Graph structures are unique and there are rarely other ways to achieve the same outcome in any other way. Working in this environment allows to focus deeply on relationships and think about problems differently. 

- Amazing in modeling relationships
- Efficient retrieval of dependencies of a specific node
- Simplifies complex abstractions
- Enables certain problems to be solved at all

Whether you choose a matrix or list representation is often a subjective choice and - for example - while the matrix provides easy deletes, a list stores edges more efficiently in the first place. It's all a trade-off.

### Downsides

This leads us to the downsides of this particular data structure:

- Unable to solve certain problems efficiently (e.g. a list of all nodes that have a certain property)
- More resource-inefficient
- Unsolved problems exist (e.g. the traveling salesman problem with a high number of cities)
- Typically requires re-thinking of a problem

With this, we can conclude this chapter about trees and their relatives after a summary:

# Summary



# Questions