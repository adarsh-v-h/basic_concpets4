fn main() {
    println!("Hello, world!");
    /*Reference: is like a pointer in that it’s an address we can follow to access the
    data stored at that address; that data is owned by some other variable.*/
    let s1:String = String::from("Nice to meet you");
    let len = cal_len(&s1); /* & represents references, allows to refer to some value without taking ownership.
    The &s1 refers to s1, but doesn't own it, therefore the s1 value is not dropped.*/
    println!("The lenght of {} is {}", s1, len);
    // mutable refernces
    let mut s2:String = String::from("Heheheh"); // first created a mutable variable
    changes(&mut s2); // we sent &mut s2 instead of just s2, allowing the function to alter the data.
    println!("new S2 : {s2}");
    let r1 = &mut s2;
    println!("{}", r1);
    // r1 now has the reference to mutable s2, 
    // note you cant have more than 1 reference to the same variable 
    // i.e let r2 = &mut s2; will give error.
    // helps RUST avoid something called data race, below is an explanation of it:
    /*  • Two or more pointers access the same data at the same time.
        • At least one of the pointers is being used to write to the data.
        • There’s no mechanism being used to synchronize access to the data.*/
    /*we can have multiple immutable refernces, but only a single mutable references, 
    and you cant even have immutable references when there is an valid, existing mutable references.*/
    // but we can divide the mutable references by scope to avoid this
    let mut s3:String = String::from("Yyoyoyoy");
    {
        let r2 = &mut s3;
        println!("{} by r2 in a scope", r2);
    }// r2 not is not valid and is dropped
    let r3 = &mut s3; // r3 is the only mutable reference in this scope
    println!("{} by r3 out of the 'r2s' scope", r3);
    // the thing is the scope of a reference starts from where they are introduced, and end when they are last used,
    // so if not i tried to initalize this
    let r4 = &s3;
    println!("{} using the new reference r4", r4);
    // this will run with no error, only if we dont use r3 any further within '}'
    // the compiler, sees that r3 was last used for a println! and never used again after that, so it drops it, making space for any new references
    //---------------------------------------------
    // we have some something like dangling references, when a pointer points a freed memory, means the data is already dropped but the reference to it still exists
    //let lost_reference = dangle(); // the returned reference should have be stored here but it couldnt
    let borr = non_dangleing();
    println!("{} is moved to borr", borr);
    //--------------------------------------------------------------
    // we also have something called slices, the def is as
    /*Slices let you reference a contiguous sequence of elements in a collection rather than the
    whole collection.*/
    // first lets look at a problem that we can solve with slices, lets consided a problem question
    /* write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
    If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.*/
    // we will create a string and pass its reference value to a function and work there.
    let s69:String = String::from("noice");
    // we will take the return value from the function in usize, which will tell us when the first space " " is occured, to the take out the first work
    let first:usize = first_word(&s69); // reference to s69
    println!("The index of the first space is {}", first);
    // even tho we can see the process is very tedious, it has the majaor flaw of not being linked to the srting as well
    // if now alter the string
    //s69.clear(); // this empties the String, making it equal to ""
    // now we have a useless variable holding the index of first space of a string that's long gone.
    // now imagine if we want second word, so we have to now find the first letter after first space and keep iterating till second space.
    // So string slice is a refernce to part of a string, we can initalize it like this
    let no = &s69[0..2]; // so the slice contains of 2 values, a pointer that points to the first value, where its starting and the length of the slice
    // and also we can drop the 0 for initial value, so we can just write [..2] = [0..2]
    let ce = &s69[3..5]; // here the pointer is pointing to idex 3 of s69 and a length of 2
    // same as the above, if we are indicating the last value we can drop it too, so it will be [3..] =[3..5] = [3..len] where, let len = s69.len()
    // so get a slice of whole string, we can just write [0..len] or [..]
    println!("{}i{}", no, ce); // prints noice
    // so lets rewrite the problem question from before
    let better_first:&str = better_first_word(&s69);
    println!("the first word is {}", better_first);
    // so here the "better_first" is a reference, therefore its in sync with the "s69" so any alterations there will be automatically altered in our slice
    /*  note: The .clear() we used before is a method that calls mutable s69 i.e &mut s69,
    and our slice is immutable but default, and from the borrowing rules we learned if there is a mutable reference that exists, we cant any more mutable or immutable referenes*/
    // also we can say String literals are Slices, thats why they are immutable, so string literals can be called as immutable reference.
    let my_string = String::from("hello world");
    // `better_first_word` works on slices of `String`s, whether partial or whole
    let _word = better_first_word(&my_string[0..6]);
    let _word = better_first_word(&my_string[..]);
    // `better_first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = better_first_word(&my_string);
    let my_string_literal = "hello world";
    // `better_first_word` works on slices of string literals, whether partial or whole
    let _word = better_first_word(&my_string_literal[0..6]);
    let _word = better_first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = better_first_word(my_string_literal);
    // we can use slices for arrays too
    let a = [1,2,3,4,5];
    let a_sl = &a[1..3];
    assert_eq!(a_sl, &[2,3]); // this checks if both values are equal during runtime, if equal ntg happens, but if not the program panics.

}

fn cal_len(s: &String) -> usize{ // &String indicates the type of the paramter is reference to a string
    s.len() // no need of tuple to return string with the len anymore
}// the s goes out of scope with affecting s1
// note: just like variables are immutable so are refernces, we can check length or any other method, but can't change the date.
// but we can have mutable refernces
fn changes(s: &mut String){ // parameter such that it can accept mutable reference String
    s.push_str(" huhahha"); // appened data
}// no return, because the change to the original value is made without moving the ownership or needing of any return value
// its like borrowing the access until the calling function's scope ends without taking away the ownership
//dangle() -> &String{ // will return a immutable refernce 
    //let s:String = String::from("Hello");// creates a variable of string type
    //&s // trying to return a reference
//} // here the drop function drops the content of s, making the memory free,
// so we are returning a reference to a dropped/freed memory, the statement from the compiler is:
/*this function's return type contains a borrowed value, but there is no value for it to be borrowed from. */
// so to solve this, we just have to move the ownership itself
fn non_dangleing() -> String{
    let s:String = String:: from("HUHUHU");
    s
}
/*So to sum it up:
• At any given time, you can have either one mutable reference or any number of
immutable references.
• References must always be valid.*/
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes(); // .as_bytes() function convertes a string to an array in byte form, which will be easier for to iterate through
    for (i, &item) in bytes.iter().enumerate(){ // iter() to iterate thorugh all one by one, enumarate is a function that returns a tuple with the first value as index and a reference to element of that index,
        //  so we have (i, &item) to take the returned value 
        if item == b' '{ // b' ' check if there is any item that is in the byter formate of an space
            return i; // if there is, return that value, and end the function here
        }
    }
    s.len()// if we didnt find any empty space, that means we only have 1 word, so it return the whole length itself.
}

fn better_first_word(s: &str) -> &str{ // this time the return value is string literal
    let bytes = s.as_bytes();// same as before converting into array of byte format
    for (i, &item) in bytes.iter().enumerate(){ // iteration and enumerate function that return a tuple of (index, reference_to_element)
        if item == b' '{ // checking for a space in byte formate
            return &s[0..i]; // returing the reference from starting to the index location of first space
            // it will be considered as slice of the first word, as the last value in the range is exclusive
            // and the function terminates as we return the value
        }
    }
    &s[..] // if no ' ' is found that means we only have 1 word and returns the whole
}