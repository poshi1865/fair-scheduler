### What the scheduler is useful for:
- You want to guarantee fairness for each "user"/"client"/"input" that uses your application. A single user shouldn't be able to block other users by sending a 1000 tasks at once, but should be still be able to process these tasks if the system has enough idle capacity.
- You have a system that is constrained by cost/compute/downstream providers (rate limits) and can process N tasks safely.
- You need a way to not stress your system, but don't want to introduce flat rate limits, and don't want your system to idle.

### Scheduler Guarantees:
1. Each user gets an equal opportunity each cycle.
2. The users who haven't had any opportunity to get their tasks processed should be prioritized.
3. The tasks which have been waiting for more turns should be prioritized over the tasks which have been waiting for less turns, provided point 2.

### Algorithm:
N: Total tasks the system can handle concurrently.
C: Current tasks the system is processing.
turns_waiting: int field on each task which represents how many scheduler cycles that task has been waiting for
cycles_since_last_served: A list of [{user_id: cycles_since_last_served}]

### Every cycle of the scheduler does the following:
1. Get a list of unique_users from the queue. Get the `cycles_since_last_served` list. Add unique users to this cycles list (ones which were not already present).
2. From the above list, get the (N-C) users which have the most `cycles_since_last_served`.
3. Pick one task with the longest turns_waiting from each user from the above list. Check if space is left after `(N-C)-K`.
4. If space is left, pick one more task (longest turns_waiting) from another user in the list until space is full.
5. This task list is the output for every cycle.

Optimization: Instead of updating every task each cycle, record a “cycle number” when the task entered the queue and compute turns_waiting = current_cycle - enqueued_cycle on demand.
