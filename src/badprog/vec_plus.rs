// https://github.com/badprog/badprog_vec_plus

use core::fmt;

//
#[derive(Debug, PartialEq)]
pub enum BP {
    Mem,
    Perf,
}

// ==========================================
//
// #[derive(Debug)]
pub struct VecPlus<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T: fmt::Debug> fmt::Debug for VecPlus<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }
}

// ==========================================
//
impl<T> Default for VecPlus<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
//
impl<T> VecPlus<T> {
    // ==========================================
    //
    pub fn new() -> Self {
        VecPlus {
            ptr: (std::ptr::null_mut()),
            len: (0),
            capacity: (0),
        }
    }

    // ==========================================
    //
    pub fn len(&self) -> usize {
        self.len
    }

    // ==========================================
    //
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // ==========================================
    //
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // ==========================================
    //
    pub fn as_slice(&self) -> &[T] {
        if self.ptr.is_null() || self.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    // ==========================================
    //
    fn write_element(&mut self, element: T) {
        unsafe {
            // Use add() instead of offset() since we're always moving forward
            let where_to_push_element = self.ptr.add(self.len);

            // Push the new element
            std::ptr::write(where_to_push_element, element);
        }
    }

    // ==========================================
    //
    fn allocation_handler(&mut self, old_capacity: usize, element: T) -> Result<(), String> {
        // Allocate new memory
        // Let's allocate for the new capacity
        let res_layout = std::alloc::Layout::array::<T>(self.capacity);
        match res_layout {
            Ok(layout) => {
                // Allocate new area
                let new_area = unsafe { std::alloc::alloc(layout) }; // new_ptr has now the new capacity
                if new_area.is_null() {
                    return Err("Allocation failed, the new_ptr is null.".to_string());
                }

                // Cast new_area to *mut T
                let new_area = new_area as *mut T;

                // If old capacity is 0, we don't have to copy old data
                // if self.ptr.is_null() && self.len == 0 {
                if self.len == 0 {
                    unsafe {
                        // isize because offset can be negative, and len is the index of the next free slot
                        let where_to_push_element = new_area.add(self.len);

                        // Push the new element
                        std::ptr::write(where_to_push_element, element);
                    }
                } else {
                    if self.ptr.is_null() {
                        return Err("Invalid state: non-zero len with null ptr".to_string());
                    }
                    unsafe {
                        // Copy old data to new area
                        std::ptr::copy_nonoverlapping(self.ptr, new_area, self.len);

                        // isize because offset can be negative, and len is the index of the next free slot
                        let where_to_push_element = new_area.add(self.len);

                        // Push the new element
                        std::ptr::write(where_to_push_element, element);
                    }

                    //
                    let old_layout = std::alloc::Layout::array::<T>(old_capacity);
                    match old_layout {
                        Ok(layout) => {
                            if !self.ptr.is_null() {
                                // Deallocate old area
                                unsafe {
                                    std::alloc::dealloc(self.ptr as *mut u8, layout);
                                }
                            }
                        }
                        // match old_layout
                        Err(err) => {
                            return Err(format!("Layout old_layout error: {:?}", err));
                        }
                    }
                }

                // Update self.ptr to point to the new area
                self.ptr = new_area;
            }
            // match layout
            Err(err) => {
                return Err(format!("Layout layout error: {:?}", err));
            }
        }

        Ok(())
    }

    // ==========================================
    //
    fn deal_with_capacity(&mut self, mode: BP) -> Result<(), String> {
        // We need to grow
        let coeff = if BP::Perf == mode { 10 } else { 1 };

        // Update capacity
        self.capacity = if self.len == 0 {
            coeff
        } else if BP::Perf == mode {
            self.capacity * coeff
        } else {
            self.capacity + coeff
        };

        Ok(())
    }

    // ==========================================
    // With mode = true, we grow by a factor of 10 (performance over memory image)
    // With mode = false, we grow by a factor of 1 (memory image over performance)
    pub fn push(&mut self, element: T, mode: BP) -> Result<(), String> {
        // Check if we need to grow
        if self.len < self.capacity {
            if self.len > 0 && self.ptr.is_null() {
                return Err("Invalid state: non-zero len with null ptr".to_string());
            }
            self.write_element(element);
        } else {
            //
            let old_capacity = self.capacity;
            self.deal_with_capacity(mode)?;

            // Handle allocation
            self.allocation_handler(old_capacity, element)?;
        }

        // Update len
        self.len += 1;

        //
        Ok(())
    }
}
