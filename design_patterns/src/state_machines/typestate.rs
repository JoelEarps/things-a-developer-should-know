struct Open;
struct Filled;
struct Settled;
struct Cancelled;


impl State for Pending { type Next = Filled; }    
impl State for Filled { type Next = Settled; }    
impl State for Settled { type Next = ... }   

/* Key points:
1. Associated types is the enforcing mechanism that chains transitions
2. Invalid state transitions will not compile
3. By doing this we make sure orders follow a chain of states and invalid transitions are not possible
*/