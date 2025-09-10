# badprog::lib_vec_plus

A new Vec version with the option to choose between performance or memory.

By default the Vec struct has a capacity of 2.  
And each time this capacity is reached, the new capacity is doubled.

With the mode BP::Memory, this VecPlus we can now add a capacity of 1 each time a new element is pushed in the vector.  
And with the mode BP::Performance, the VecPlus will multiply the capacity by 10 each time the max capacity is reached.

## BP::Memory
For example, with the first mode (BP::Memory), if the vector is empty and if we add an element, the new capacity will be 1.  
If we add another element, the capapcity will become 2.  
The next element will make the capacity become 3.
And so on.


## BP::Performance
For example, with the latter mode, if the current capacity is 1, the next push will make the capacity multiply by 10.  

So the new capacity is now 10.  
And if this capacity of 10 is reached, the next element pushed in the vector, will make the capacity multiplied again by 10.  

Then the current capapcity is now 100.  

After the 100th elements pushed, the capacity will jump to 1000.  

And so on.

