/*---------------------------------------------------------
 *  Set Theory
 *---------------------------------------------------------*/

/* *****************************************************************************
 *  Name:    Alan Turing
 *  NetID:   aturing
 *  Precept: P00
 *
 *  Description:  Exemplifies the way sets are operated 
 *                in the Rust programming language
 *
 *  Written:       9/09/2020
 *  Last updated:  9/11/2020
 *
 *  % Rut 170201-Conjuntos.rs
 *  % Rust Set Theory
 *  Define three sets
 *  Printing Set
 *  Belonging to a set
 *  Transform to sets
 *  Set Operations
 *
 **************************************************************************** */


use std::collections::HashSet;

fn main()
{
	//Define three sets
    let a: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let b: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    let c: HashSet<i32> = HashSet::new();
    
    //Printing Set
    println!("{:?}", &a);
    println!("{:?}", &b);
    println!("{:?}", &c);
    
    //Call Functions
    pertenencia();
    transformarConj();
    quitarElemento();
    clearSet();
    copiar();
    agregar();
    union();
    intereseccion();
    diferencia();
    simetrica();
    subconjunto();
    superconjunto();
}

//Pertenencia
fn pertenencia()
{
    let a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    //1 in A
    println!("{}",a.contains(&1));
    //1 not in A 
    println!("{}",!a.contains(&1));
    //10 in A 
    println!("{}",a.contains(&10));
    //10 not in A 
    println!("{}",!a.contains(&10));
}

/*  --- Convert String to set ---
 *  Strings in Rust do not have their own iteration method,
 *  a conventional iteration with a "for" loop must be used
 *  to obtain their components.
*/
fn convierteCadena(cad: &str) -> HashSet<char>
{
    // A set of type Char is created
    let mut set: HashSet<char> = HashSet::new();
    
    // It iterates through each element of the chain
    for letra in cad.chars() 
    {
        // Each element of the string is inserted into the set
        set.insert(letra);
    }
    // Returns the set with the elements of the string
    return set;
}


fn transformarConj()
{
    // Structure declaration
    let a = vec![1, 2, 3, 4, 5]; // Vector
    let b = [1, 2, 3, 4, 5];     // Array
    let c = "Hola Mundo";        // String
    
    // Set transformation
    let aConj: HashSet<_>    = a.iter().collect(); // Transform set A
    let bConj: HashSet<_>    = b.iter().collect(); // Transform set B
    let cConj: HashSet<char> = convierteCadena(c); // Transform set C
    
    // Prints the sets
    println!("The set is A: {:?}", &aConj); // Set A
    println!("The set is B: {:?}", &bConj); // Set B
    println!("The set is C: {:?}", &cConj); // Set C
    
}

// Remove an item from the set
fn quitarElemento()
{
    // Set declaration
    let mut a: HashSet<_> = [0,1,2,3,4,5].iter().collect();
    
    // Remove item "2" from the set
    a.remove(&2);
    
    // Prints the set
    println!("The set after to delete: {:?}", &a);
}


//Quitar todos los elementos de un conjunto
fn clearSet()
{
    // Set declaration
    let mut a: HashSet<_> = [0,1,2,3,4,5].iter().collect();
    
    // Remove all items from the set
    a.clear();
    
    // Prints the set
    println!("The set clear: {:?}", &a)
}
   
//Copiar un conjunto
fn copiar()
{
    // Set declaration
    let a: HashSet<_> = [0,1,2,3,4,5].iter().collect();
    
    // Clone set A into set B
    let b: HashSet<_> = a.clone();
    
    // Prints the set
    println!("Set A = {:?} compare set B = {:?}", &a,&b)
}
 
//Agregar un elemento
fn agregar()
{
    // Set declaration
    let mut b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    
    // inserts item "987" into the set
    b.insert(&987) ;
    
    // Prints the set
    println!("The new set B = {:?}", &b) ;
}
////////////////////////////////Set Operations//////////////////////////////


//Union
fn union()
{
    // Set declaration
    let a: HashSet<_> = [1,2,3,4,5].iter().collect(); 
    let b: HashSet<_> = [3,4,5,6,7].iter().collect();
    
    // Prints the union of set A and set B
    println!("The union = {:?}", a.union(&b));
}

//Interseccion
fn intereseccion()
{
    // Set declaration
    let a: HashSet<_> = [1,2,3,4,5].iter().collect();
    let b: HashSet<_> = [3,4,5,6,7].iter().collect();
    
    // Prints the intersection of set A and set B
    println!("The intersection = {:?}",a.intersection(&b));
}

//Diferencia
fn diferencia()
{
    // Set declaration
    let a: HashSet<_> = [1,2,3,4,5].iter().collect(); 
    let b: HashSet<_> = [3,4,5,6,7].iter().collect(); 
    
    // Prints the difference of set A and set B
    println!("The diference = {:?}", a.difference(&b))
}

//Diferencia simetrica
fn simetrica()
{
    // Set declaration
    let a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    let c: HashSet<_> = [].iter().collect();
    
    // Prints the symmetric difference of set A and set B
    println!("The symmetric difference = {:?}",a.symmetric_difference(&b));
    
    // Prints the symmetric difference of set B and set A
    println!("The symmetric difference = {:?}",b.symmetric_difference(&a));
    
    // Prints the symmetric difference of set A and set C
    println!("The symmetric difference = {:?}",a.symmetric_difference(&c));
    
    // Prints the symmetric difference of set B and set C
    println!("The symmetric difference = {:?}",b.symmetric_difference(&c));
}

//Subconjunto
fn subconjunto()
{
    // Set declaration
    let b: HashSet<_> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect(); 
    let a: HashSet<_> = [1,2,3,4,5].iter().collect(); 
    
    // Prints if set A is subset of set B
    println!("The subset = {}",a.is_subset(&b) );
    
    // Prints if set B is subset of set A
    println!("The subset = {}",b.is_subset(&a) );
}

//Superconjunto
fn superconjunto()
{
    // Set declaration
    let b: HashSet<_> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect();
    let a: HashSet<_> = [1,2,3,4,5].iter().collect();
    
    // Prints if set B is superset of set A
    println!("The superset = {}",b.is_superset(&a) );
    
    // Prints if set A is superset of set B
    println!("The supersrt = {}",a.is_superset(&b) );
}

/*
Rust has Arrays and Vectors. The empty Array or vector looks like []. 
The following has one item ["a"] and so is [3]. Here is one with 3 items [3,14,-2]. 
They are strictly typed so they cannot contain values ??of different types in the same list
*/