use std::collections::HashSet;

fn main()
{
	//Define three sets
    let a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    let c: HashSet<i32> = HashSet::new();
    
    //Imprimir
    println!("{:?}", &a);
    println!("{:?}", &b);
    println!("{:?}", &c);
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

//Convertir a un conjunto
fn convierteCadena(cad: &str) -> HashSet<char>
{
    let mut set: HashSet<char> = HashSet::new();
    for letra in cad.chars()
    {
        set.insert(letra);
    }
    
    return set;
}


fn transformarConj()
{
    let a = vec![1, 2, 3, 4, 5]; //Vector
    let aConj: HashSet<_> = a.iter().collect();
    println!("The set is A: {:?}", &aConj);
    
    let b = [1, 2, 3, 4, 5]; //Array
    let bConj: HashSet<_> = b.iter().collect();
    println!("The set is B: {:?}", &bConj);
    
    let c = "Hola Mundo"; //Cadena
    let cConj: HashSet<char> = convierteCadena(c);
    println!("The set is C: {:?}", &cConj);
    
}

//#Quitar un elemento al conjunto
fn quitarElemento()
{
    let mut a: HashSet<_> = [0,1,2,3,4,5].iter().collect();
    a.remove(&2);
    println!("The set after to delete: {:?}", &a);
}


//Quitar todos los elementos de un conjunto
fn clearSet()
{
    let mut a: HashSet<_> = [0,1,2,3,4,5].iter().collect();
    a.clear();
    println!("The set clear: {:?}", &a)
}
   
//Copiar un conjunto
fn copiar()
{
    let a: HashSet<_> = [0,1,2,3,4,5].iter().collect();
    let b: HashSet<_> = a.clone();
    println!("Set A = {:?} compare set B = {:?}", &a,&b)
}
 
//Agregar un elemento
fn agregar()
{
    let mut b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    b.insert(&987) ;
    println!("The new set B = {:?}", &b) ;
}
////////////////////////////////Set Operations//////////////////////////////


//Unión
fn union()
{
    let a: HashSet<_> =[1,2,3,4,5].iter().collect(); 
    let b: HashSet<_> =[3,4,5,6,7].iter().collect();
    println!("The union = {:?}", a.union(&b));
}

//Interseccion
fn intereseccion()
{
    let a: HashSet<_> = [1,2,3,4,5].iter().collect();
    let b: HashSet<_> = [3,4,5,6,7].iter().collect();
    println!("The intersection = {:?}",a.intersection(&b));
}

//Diferencia
fn diferencia()
{
    let a: HashSet<_> = [1,2,3,4,5].iter().collect(); 
    let b: HashSet<_> = [3,4,5,6,7].iter().collect(); 
    println!("The diference = {:?}", a.difference(&b))
}

//Diferencia simetrica
fn simetrica()
{
    let a: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    let c: HashSet<_> = [].iter().collect();
    
    println!("The symmetric_difference = {:?}",a.symmetric_difference(&b));
    println!("The symmetric_difference = {:?}",b.symmetric_difference(&a));
    println!("The symmetric_difference = {:?}",a.symmetric_difference(&c));
    println!("The symmetric_difference = {:?}",b.symmetric_difference(&c));
}

//Subconjunto
fn subconjunto()
{
    let b: HashSet<_> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect(); 
    let a: HashSet<_> = [1,2,3,4,5].iter().collect(); 
    println!("The subset = {}",a.is_subset(&b) );
    println!("The subset = {}",b.is_subset(&a) );
}

//Superconjunto
fn superconjunto()
{
    let b: HashSet<_> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().collect();
    let a: HashSet<_> = [1,2,3,4,5].iter().collect();
    println!("The superset = {}",b.is_superset(&a) );
    println!("The supersrt = {}",a.is_superset(&b) );
}