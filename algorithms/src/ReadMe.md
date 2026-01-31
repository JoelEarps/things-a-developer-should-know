# Algorithms

## DSA (Data Structures and Algorithms)

### Data Structures

A data structure is a way to store data.

We structure data in different ways depending on what we have to do with it. They give us the ability to manage large amounts of data efficiently depending on the use case. Examples of this are databases and internet indexing services.

They are essential to creating fast algorithms by managing and organising data in the least complex and most accessible format.

There are two different types of data structures:

1. **Primitive** - Basic data structures used to represent a single value (e.g., bool, chars, ints, and fixed points).
2. **Abstract Data Structure** - Higher level structures that are built on primitive data for more complex and specialised operations (e.g., arrays, linked lists, stacks, trees, and graphs).

### Algorithms for Data

An algorithm is a step-by-step instruction to solve a particular problem.

Data structures and Algorithms (DSA) go hand in hand: DSA is about finding efficient ways to store and retrieve data to perform operations on data and solve specific problems.

To better understand algorithms you can:

1. Decide which structure is best for any given situation
2. Make programs run faster and use less memory
3. Understand how to approach complex problems and solve them in a systematic way

## Common Terminology

- **Algorithm** - A set of step-by-step instructions to solve a problem
- **Data Structure** - A way of organising data so it can be used efficiently
- **Time Complexity** - A measure of the amount of time an algorithm takes to run, depending on the data the algorithm is working on
- **Space Complexity** - A measure of the amount of memory an algorithm uses, depending on the amount of data the algorithm is working on
- **Big O Notation** - Mathematical notation that describes the limiting behaviour of a function when the argument tends towards a particular value or infinity
- **Recursion** - A program or function that calls itself
- **Divide and Conquer** - A problem-solving approach where you break the problem into manageable smaller ones and then combine the solutions
- **Brute Force** - A simple, straightforward solution that works by trying all possible solutions and then choosing the best one

## Algorithm Types, Complexities, and When to Use Them

### 1. Search Algorithms

#### Linear Search

- **Time Complexity**: O(n)
- **Space Complexity**: O(1)
- **When to Use**:
  - Small datasets
  - Unsorted data
  - When you need to find the first occurrence
  - Simple implementation is acceptable

#### Binary Search

- **Time Complexity**: O(log n)
- **Space Complexity**: O(1) iterative, O(log n) recursive
- **When to Use**:
  - Sorted arrays or lists
  - Large datasets where O(n) is too slow
  - When you need to find an element or determine if it exists
  - Can be adapted for finding insertion points, ranges, etc.

#### Two Pointers

- **Time Complexity**: O(n)
- **Space Complexity**: O(1)
- **When to Use**:
  - Sorted arrays
  - Finding pairs that sum to a target
  - Removing duplicates
  - Merging sorted arrays
  - Palindrome checking

#### Depth-First Search (DFS)

- **Time Complexity**: O(V + E) where V = vertices, E = edges
- **Space Complexity**: O(V) for recursion stack
- **When to Use**:
  - Exploring all paths in a graph/tree
  - Topological sorting
  - Finding connected components
  - Solving puzzles (mazes, sudoku)
  - When you need to go deep before exploring wide

#### Breadth-First Search (BFS)

- **Time Complexity**: O(V + E) where V = vertices, E = edges
- **Space Complexity**: O(V) for queue
- **When to Use**:
  - Finding shortest path in unweighted graphs
  - Level-order traversal
  - Finding minimum spanning tree (unweighted)
  - Social network analysis (degrees of separation)
  - When you need to explore level by level

### 2. Sort Algorithms

#### Bubble Sort

- **Time Complexity**: O(n²) average and worst case, O(n) best case (already sorted)
- **Space Complexity**: O(1)
- **When to Use**:
  - Educational purposes
  - Very small datasets (< 10 elements)
  - When simplicity is more important than performance
  - **Avoid** for production code with larger datasets

#### Merge Sort

- **Time Complexity**: O(n log n) in all cases
- **Space Complexity**: O(n)
- **When to Use**:
  - When you need a stable sort (preserves relative order of equal elements)
  - Large datasets where consistent performance matters
  - External sorting (sorting data that doesn't fit in memory)
  - When worst-case O(n log n) is required
  - Linked lists (efficient for linked structures)

#### Quick Sort

- **Time Complexity**: O(n log n) average, O(n²) worst case
- **Space Complexity**: O(log n) average, O(n) worst case
- **When to Use**:
  - General-purpose sorting
  - When average-case performance is more important than worst-case
  - In-memory sorting of arrays
  - When stability is not required
  - **Avoid** when worst-case O(n²) is unacceptable

#### Insertion Sort

- **Time Complexity**: O(n²) average and worst case, O(n) best case
- **Space Complexity**: O(1)
- **When to Use**:
  - Small datasets (< 50 elements)
  - Nearly sorted data
  - As a subroutine in more complex algorithms (e.g., Timsort)
  - When simplicity and low overhead matter

#### Selection Sort

- **Time Complexity**: O(n²) in all cases
- **Space Complexity**: O(1)
- **When to Use**:
  - Educational purposes
  - Very small datasets
  - When minimizing writes to memory is important
  - **Avoid** for production code

#### Heap Sort

- **Time Complexity**: O(n log n) in all cases
- **Space Complexity**: O(1)
- **When to Use**:
  - When you need guaranteed O(n log n) performance
  - When memory is limited (in-place sorting)
  - Finding k largest/smallest elements
  - Priority queue implementations

#### Topological Sort

- **Time Complexity**: O(V + E)
- **Space Complexity**: O(V)
- **When to Use**:
  - Directed acyclic graphs (DAGs)
  - Task scheduling with dependencies
  - Build systems (determining build order)
  - Course prerequisites
  - Event ordering

### 3. Graph Algorithms

#### Dijkstra's Algorithm

- **Time Complexity**: O((V + E) log V) with binary heap, O(V²) with array
- **Space Complexity**: O(V)
- **When to Use**:
  - Finding shortest path in weighted graphs
  - Non-negative edge weights only
  - Single-source shortest path problems
  - GPS navigation systems
  - Network routing protocols

#### Bellman-Ford Algorithm

- **Time Complexity**: O(VE)
- **Space Complexity**: O(V)
- **When to Use**:
  - Graphs with negative edge weights
  - Detecting negative cycles
  - When Dijkstra's cannot be used (negative weights)
  - Distance-vector routing protocols
  - **Slower than Dijkstra's** but more versatile

#### A* (A-Star) Algorithm

- **Time Complexity**: O(b^d) where b = branching factor, d = depth
- **Space Complexity**: O(b^d)
- **When to Use**:
  - Pathfinding in games and maps
  - When you have a good heuristic function
  - Finding shortest path with additional information
  - More efficient than Dijkstra's when heuristic is admissible
  - AI pathfinding

#### Floyd-Warshall Algorithm

- **Time Complexity**: O(V³)
- **Space Complexity**: O(V²)
- **When to Use**:
  - All-pairs shortest path problems
  - Small to medium graphs (V < 400)
  - When you need distances between all pairs of vertices
  - Transitive closure problems
  - **Avoid** for large graphs due to cubic complexity

### 4. Greedy Algorithms

#### Greedy Algorithm Characteristics

- **Time Complexity**: Varies by problem (often O(n log n) due to sorting)
- **Space Complexity**: Usually O(1) or O(n)
- **When to Use**:
  - Optimization problems with optimal substructure
  - When locally optimal choices lead to global optimum
  - Activity selection problems
  - Fractional knapsack
  - Minimum spanning tree (Kruskal's, Prim's)
  - Huffman coding
  - **Note**: Greedy doesn't always give optimal solution - verify the problem has greedy choice property

#### Common Greedy Problems

- **Activity Selection**: O(n log n) - scheduling non-conflicting activities
- **Fractional Knapsack**: O(n log n) - maximize value with weight constraints
- **Kruskal's MST**: O(E log E) - minimum spanning tree
- **Prim's MST**: O(E log V) - minimum spanning tree
- **Huffman Coding**: O(n log n) - data compression

### 5. Dynamic Programming

#### Dynamic Programming Characteristics

- **Time Complexity**: Typically O(n) to O(n²) or O(n³) depending on problem
- **Space Complexity**: O(n) to O(n²) typically
- **When to Use**:
  - Problems with overlapping subproblems
  - Optimal substructure property
  - When recursive solution has repeated calculations
  - Optimization problems
  - Counting problems

#### Common DP Problems

- **Fibonacci**: O(n) time, O(1) space (optimized)
- **Longest Common Subsequence**: O(mn) time, O(mn) space
- **Knapsack (0/1)**: O(nW) time, O(nW) space
- **Edit Distance**: O(mn) time, O(mn) space
- **Coin Change**: O(n × amount) time, O(amount) space

### 6. Divide and Conquer

#### Divide and Conquer Characteristics

- **Time Complexity**: Often O(n log n)
- **Space Complexity**: O(log n) to O(n)
- **When to Use**:
  - Problems that can be broken into similar subproblems
  - When subproblems are independent
  - Merge sort, quick sort
  - Binary search
  - Finding maximum subarray
  - Matrix multiplication (Strassen's)

### Decision Guide: Which Algorithm to Choose

#### For Searching

- **Sorted data + need fast lookup** → Binary Search
- **Unsorted data + small dataset** → Linear Search
- **Finding pairs/ranges in sorted data** → Two Pointers
- **Graph/tree traversal** → DFS or BFS (see graph section)

#### For Sorting

- **General purpose, average performance** → Quick Sort
- **Need stability + consistent performance** → Merge Sort
- **Small dataset (< 50)** → Insertion Sort
- **Nearly sorted data** → Insertion Sort
- **Need guaranteed O(n log n)** → Heap Sort or Merge Sort
- **Dependencies/ordering** → Topological Sort

#### For Graphs

- **Shortest path, non-negative weights** → Dijkstra's
- **Shortest path, negative weights possible** → Bellman-Ford
- **Pathfinding with heuristic** → A*
- **All-pairs shortest path** → Floyd-Warshall
- **Explore all paths, deep first** → DFS
- **Level-by-level, shortest unweighted path** → BFS

#### For Optimization

- **Greedy property holds** → Greedy Algorithm
- **Overlapping subproblems** → Dynamic Programming
- **Can break into independent subproblems** → Divide and Conquer
