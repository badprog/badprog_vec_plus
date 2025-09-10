# badprog::lib_vec_plus

An enhanced Vec implementation with 2 growth strategies, allowing to choose between performance or memory.

By default the Vec struct has a capacity of 2.  
And each time this capacity is reached, the new capacity is doubled.

With the **BP::Memory** mode, VecPlus increases the capacity by 1 each time a new element is pushed.  
With the **BP::Performance** mode, VecPlus multiplies the capacity by 10 each time the maximum capacity is reached.

## BP::Memory
For example, with the first mode (BP::Memory), if the vector is empty and if we add an element, the new capacity will be 1.  
If we add another element, the capacity will be 2.  
The next element will make the capacity become 3.
And so on.


## BP::Performance
For example, with the latter mode, if the current capacity is 1, the next push will make the capacity multiply by 10.  

So the new capacity is now 10.  
And if this capacity of 10 is reached, the next element pushed in the vector, will make the capacity multiplied by 10 again.  

Then the current capacity is now 100.  

After the 100th element pushed, the capacity will jump to 1000.  

And so on.

