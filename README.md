### What the scheduler is useful for:
- You want to guarantee fairness for each "user"/"client" that uses your application. A single user shouldn't be able to block other users by sending a huge amount of tasks at once, but should be still be able to process these tasks if the system has enough idle capacity.
- You have a system that is constrained by cost/compute/downstream providers (rate limits), but you know it can process `N` tasks safely.
- You need a way to not stress your system, but don't want to introduce flat rate limits, and don't want your system to idle.

### Scheduler Guarantees:
1. Each user gets an equal opportunity each cycle.
2. The users who haven't had any opportunity to get their tasks processed should be prioritized.
3. The tasks which have been waiting for more turns should be prioritized over the tasks which have been waiting for less turns, provided point 2.

### Algorithm:
`N`: Total tasks the system can handle concurrently.
`C`: Current tasks the system is processing.
turns_waiting: int field on each task which represents how many scheduler cycles that task has been waiting for

#### Every cycle of the scheduler does the following:
1. Get a list of all users from the queue. From this, pick `N-C` users who have been waiting the longest amount of cycles.
2. Pick one task with the longest turns_waiting from each user from the above list. Check if space is left after `(N-C)-K`.
3. If space is left, repeat 1 and 2.
4. This task list is the output for every cycle.
