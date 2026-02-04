
// implementing split_at_mut
//
// apparently this is a standard library function (it's actually a method, we'll write it as a
// function". it:
// 
// Inputs:      a mutable slice, an index to split it at
// Outputs:     two slices, split at the given index
// Side FX:     None

// a slice is a data type -- it is a reference to "a contiguous sequence of elements in a
// collection"
// A "collection" is stuff like string, vector (calling them arrays is illegal i guess), hash map


pub fn split_at_mut (whole: &[i32], split: usize) -> (&mut [i32], &mut[i32]) {
    // simple error checking
    assert!(split <= whole.len());

    // makes a pointer to the whole input thing
    let whole_ptr = whole.as_mut_ptr();
    
    
   let ret_tup = unsafe {
        // the &mut part means "a mutable reference"
        // means "here, you get to look at this part of a data object i own, and even edit it"
        //
        // the [..mid], [mid..] is how slice comprehension works. it means "before mid" and "after
        // mid"
        

        // okay, so this wont work. i couulllldddd...... copy 
        (&mut whole[..split], &mut whole[split..])
        

   };
        return ret_tup;
}
