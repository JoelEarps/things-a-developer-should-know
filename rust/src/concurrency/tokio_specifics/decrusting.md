# Decrusting tokio

https://www.youtube.com/watch?v=o2ob8zkeq2s&t=8364s
With details from https://docs.rs/tokio/1.48.0/tokio/runtime/index.html

On the face it takes things that implement the future and returns the thing that is in the associated output type.
Runs future and returns the output that the future output type.
Tokio is an event driven I/O platform for writing async rust

4 part splits:
1. The tokio run time - how does it execute, worker thread, worker pool, blocking and i.e. the schedular
2. Resources - anything that has to do with io, threading processing, file system, timers
3. Utilities - what additional useful features tokio provides you with, coord and sync between futures, select macro
4. Common complications - things that are easy to shoot self in foot with.

## Tokio runtime

In tokio, submodules and one is runtime. It is the heart of the tokio - you pass it futures and it returns the associated type.
The run time is an i/o event loop called the driver which drives I/o resources and.

The schedulers job is to take futures that come in, call the poll method and at some point when ready then tokio passes the T or drops the value.
When the future returns pending, the future gets put back into the run time for poll to be called again at some point.

Two schedulars in tokio:
1. Multi thread schedular - creates one OS thread for every cpu core you have. Different worker threads can call poll on different futures at the same time, so you get parralellism to utilise resources of the machine. In comparison to switching on the same thread and have apparent concurrency.
2. Current thread schedular - Does not start any threads, it uses the current thread, starts a tokio run time on the current thread that has the IO integration. Stores all locally on the current thread and ultimately blocks current thread till work is done.

### Runtime Interface

block_on - straightforward way to start a run time on the current thread. It blocks until the task is done and then the
spawn - takes a future and put the futures onto the queue of tasks for the current runtime, but does not execute, returns a join handle. Similar to thread spawn
Join handle - can be used to abort the task or you can await it (as it implements future). So you can yeiled the value and get it back, dropping it means nothing happens.
block on - blocks the thread until it completes.
Every future spawned onto a run time becomes a task. But every future in a run time is not a task. Only futures that are passed to spawn become tasks, it does not know about all the inner futures. Tokio schedular only knows about the top level task.

#### Current thread run time

If you call spawn nothing happens, you need to call block on the current thread, it looks at the set of tasks, execute until they yield poll ready or poll pending, depending on the task queue. It then does it in a loop untill all the tasks are ready and done.

Use case:
1. Testing - Multi thread schedular with only one thread
2. Limit the amount of movement between different CPU's.

Two FIFO queues ready to be scheduled tasks - global and local. Only pick tasks from global if local is empty, or if hasn't pick from global queue 31 times in a row.
global_queue_interval setting can change this number.
Checks for new IO or timer events when scheduled 61 tasks in a row once again this can be changed with event interval.


#### Multi threaded run time

Pool of worker threads for every cpu core, the worker thread has its own local queue of tasks, then also a global queue they can all read from. Every worker thread is in a loop, and it looks at its own queue, tries to do work and if nothing there looks at the global queue and then ir tries to do work, if none there it can actually steal work from other queues.

Futures must be send in order for tokio to be able to do this load distribution. It uses the same interfaces for multi and current thread so therefore all futures you create must implement send.

Std lib thread spawn - OS system level thread, no async, tokio lets you give it a future.
Tokio main - just creates a multi threaded run time with everything enabled and blocks on so everything becomes a sub task of main.

So in terms of queues, you have a runnable and a none runnable queue to make sure you don't re run jobs and also know when to run and not run things.

Kernel CPU schedular will also implement work stealing. OS does not know about work stealing, also kernel level context switching is very expensive.
Green threading is also much less expensive and provides a lot of benefits that don't require expensive OS and hardware based.

Multi threaded runtime has a number of worker threads created on start up each with a worker queue, there is also one global queue. Local queue can fit 256 taks at most, if more than 256 threads are added then half are moved ot the global queue to make space.

Run time will choose to schedule tasks from the local queue and will only pick a task from the global queue. Will only pick a task from the global queue if the local queue is empty or if it has picked from the local queue more than setting `global_queue_interval`.
If local queue and global queue are empty then the thread can try and steal work from another threads queue. Move half the tasks in one local queue to another local queue.
Check for new IOs/ timers if scheduled more than 61 tasks or there are no new tasks. If a tasks wakes another task it is added to the LIFO slot of the thread rather than the queue and replaces any task in there.

##### Cooperative scheduling

##### Lifo Slot explained

### Detailed run time behaviour

At most basic level, runtime collection tasks need to be scehduled. Repeatedly move task from collection and schedule it by calling poll. Collection empty will go to sleep until tasks added to collection.

Total number of tasks does not grow without bounds, no task blocking thread, guaranteed tasks scheduled fairly.
1. Some number MAX TASKS and at any time number of tasks does not exceed this.
2. Some number max schedule, calling poll on tasks returns within max schedule time units.

Other than this no guarantee tasks scheduled fairly, tasks only scheduled if they have been woken by their waker. Runtime can also spontaneously wake tasks that haven't woken.

### IO timers

Runtime also manages IO resources and timers. Does this my checking if any IOs or timers that are ready and waking them to be scheduled.
Periodic checks between tasks, and happens within some unit of time.

## Blocking

Given you have N worker threads, you can run into trouble when you then run something that calls something that blocks the current thread. This can also block an OS thread, which blocks the entire worker thread. You don't want to call an operation that causes the blocking, this can be:
1. OS level calls
2. Data processing

If you have compute that takes more than 100ms you should probably do something that blocks in place or pin the task.
If you take a sts mutex, normally it is super fast. However multiple threads, accessing the same lock or trying to lock, then you can hold all worker threads up.


Two ways to deal with this problem:
1. Spawn blocking - two thread pools, one for async tasks, one is a pool for threads that are expected to block. You can spawn blocking threads to pin tasks, more efficient to OS threads, it can resuse blocking threads.
2. Block in place - tokio takes all of the state that the worker thread has, saves it somewhere it and becomes a none worker thread and tokio starts a new worker thread in place. It continues executing blocking code and doesn't interrupt the OS.

## Shutdown

When future of run time returns back to its caller, it will yield control back to the runtime, with run time when main future exits everything else just keeps dropped.

## Send bounds and local set

Local set - a set of tasks that all guaranteed to be ran on the same thread
Spawn local - adds future to local set, does not move between threads with work stealing

It cannot be used anywhere, as we said early tokio only sees top level tasks, so therefore starting it deep down in a stack of futures. Can only be used and passed to block on, run until, or you have to spawn your own thread with its own current thread run time. With tasks to it sent via a channel.

## Tokio mutex vs Std lib mutex

Lock methods on tokio mutex is async
But that doesn't mean you shouldn't use them everywhere
When you take a std lib mutex, you might block the current thread, do the work and then release. There is not a lot of risk there, tokio doesn't care it only cares about time being held up. A std lib mutex is really efficent.
Tokio mutex in order to make lock and unlock comes with a lot of overhead, its not efficeint.

Where you use it:
1. A long time to lock and unlock the mutex - so you call lock and wait blocking a thread
2.  Call the lock, then you want to hold it across an await point std lib mutex not send, so you need to give up the lock and pass the mutex then tokio knows to put the task to sleep and put it back.

So use std lib when you have short criticial sections of code that need access to the data behind the mutex, otherwise use the tokio mutex.

Tokio lock guard is not dropped whilst we await. Lock is held during the entire call.

## Q and As

Multiple run times vs one run time per task set - might get performance benfits here but the tasks have to be the same really and hyper otpimised to get better cache performance on cpus. BUT YOU NEED BENCH MARKING AS MANAGING MULTIPLE RUNTIMES IS A BALL ACHE.

Local set does not have performance penalty.

Tokio console

Spawn blocking always happens on a seperate thread.
No task priorities in tokio
Worker thread steal from another blocked worker thread - queue of worker can be but currently blocked task cannot be stolen from.
Cannot share tasks between run times, task is a single instance of the future trait.
Stealing does have some over head

NUMA Nodes

## Tokio resources

The run time is a away to poll and run instances, however you ideally want things to interact with the real world, e.g.
1. TCP/ UDP
2. Files
3. Signal processing

In future stack, normally you will have the low level interaction with the resources.
Top level call to poll on any given future, it is given a context.
Context only has one method of interest - wakers, when ready the future calls wake which signals tokio to run from the none runnable queue into the runnable queue.
Run time has an IO event loop - they look for event that might make progress and then call the waker

Runtime is running schedular and io loop at the same time. The schedular runs tasks that a runnable from the runnable queue, the event loop moves things from runnable to non runnable by calling wake.

## Tokio fs nauances

Tokios async implementation of how to access the file system. A lot of OS file system code does not support async operations.
Depedning on the OS - tokio will often use a blocking thread to give you file system access.
If file system is on a critical path you want to make that blocking

## Tokio process

When you drop the handle to a child process, the process is not terminated - it will continue executing. Kill on drop, but should be aware when it happens you need to handle it.

## Tokio IO

There are extension traits that make it easier to work with standard sync traits
If you find yourself putting an IO resource behind a mutex you are probably leaving performance behind - but you are better off sending one task that owns the TCP stream

## Tokio Streams

Sink and Stream - the async version of channel send and iterators.
Framing - string of bytes is one value, working with a codec.
