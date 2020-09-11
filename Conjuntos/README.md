# Datos personales

•	**Institución:** Universidad politécnica de San Luis Potosí

•	**Matricula:** 170201

•	**Alumno:** Juan Humberto Herrera Martínez

•	**Carrera:** Ingeniería en Tecnologías de la Información

•	**Materia:** Teoría computacional


# Objetivos 
1.	En este programa se aborda lo esencial dentro de las operaciones de conjuntos en el lenguaje de programación Rust.
2.	Entre las funciones básicas y operaciones a utilizar se encuentran

### Funciones básicas:
  Definir un conjunto, 
  Mostrar conjunto en la terminal, 
  Pertenencia de un elemento a un conjunto, 
  Transformar colecciones en Conjuntos, 
  Quitar elemento de un conjunto, 
  Remover todos los elementos de un conjunto, 
  Copiar un conjunto en otro, 
  Agregar elemento a un conjunto.
  
### Operaciones:
  Unión, 
  Intersección, 
  Diferencia, 
  Diferencia simétrica, 
  subconjunto, 
  superconjunto.

# Como solucionaste el problema

La librería de colección estándar de Rust (std::collections) proporciona implementaciones eficientes de las estructuras de datos de programación de propósito general más comunes. Al utilizar las implementaciones estándar, debería ser posible que dos bibliotecas se comuniquen sin una conversión de datos significativa.
Vec o HashMap son dos colecciones cubren la mayoría de los casos de uso para el almacenamiento y procesamiento de datos genéricos. Todas las demás colecciones de la biblioteca estándar tienen casos de uso específicos en los que son la opción óptima, pero estos casos son un nicho límite en comparación. 
Por lo anterior es que se acudió a utilizar la colección HashSet que se rige por la siguiente prioridad: 

k1 == k2 -> hash (k1) == hash (k2)

Es decir que, si dos claves son iguales, sus valores hash deben ser iguales. Por esto,  un elemento que entra a la colección obtiene su posición determinada por el Hash code, determinando su igualdad, y de esta manera cumplir .

### Comandos para ejecutar las operaciones.
•	*Definir un conjunto*

```Rust
//Define three sets
    let a: HashSet<_>   = [1, 2, 3, 4, 5].iter().collect();
    let b: HashSet<_>   = [3, 4, 5, 6, 7].iter().collect();
    let c: HashSet<i32> = HashSet::new();
```
 
•	*Mostrar conjunto en la terminal*

```Rust
    //Printing Set
    println!("{:?}", &a);
    println!("{:?}", &b);
    println!("{:?}", &c);
```

•	*Pertenencia de un elemento a un conjunto*

```Rust
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
```

•	*Transformar colecciones en Conjuntos*

```Rust
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
```

•	*Quitar elemento de un conjunto*


```Rust
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
```

•	*Remover todos los elementos de un conjunto*


```Rust
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
```
   
•	*Copiar un conjunto en otro*

```Rust
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
```

•	*Agregar elemento a un conjunto*


```Rust//Agregar un elemento
fn agregar()
{
    // Set declaration
    let mut b: HashSet<_> = [3, 4, 5, 6, 7].iter().collect();
    
    // inserts item "987" into the set
    b.insert(&987) ;
    
    // Prints the set
    println!("The new set B = {:?}", &b) ;
}
```

•	*Unión*


```Rust
//Union
fn union()
{
    // Set declaration
    let a: HashSet<_> = [1,2,3,4,5].iter().collect(); 
    let b: HashSet<_> = [3,4,5,6,7].iter().collect();
    
    // Prints the union of set A and set B
    println!("The union = {:?}", a.union(&b));
}
```

•	*Intersección*


```Rust
//Interseccion
fn intereseccion()
{
    // Set declaration
    let a: HashSet<_> = [1,2,3,4,5].iter().collect();
    let b: HashSet<_> = [3,4,5,6,7].iter().collect();
    
    // Prints the intersection of set A and set B
    println!("The intersection = {:?}",a.intersection(&b));
}
```

•	*Diferencia*


```Rust
//Diferencia
fn diferencia()
{
    // Set declaration
    let a: HashSet<_> = [1,2,3,4,5].iter().collect(); 
    let b: HashSet<_> = [3,4,5,6,7].iter().collect(); 
    
    // Prints the difference of set A and set B
    println!("The diference = {:?}", a.difference(&b))
}
```

•	*Diferencia simétrica*


```Rust
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
```

•	*Subconjunto*


```Rust
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
```

•	*Superconjunto*


```Rust
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
```

# Problemas y soluciones al programar 
El tipo String dentro de Rust, no cuenta con un método de iteración dentro de su estructura, a diferencia de los vectores y arreglos, por lo que, para introducir una cadena de caracteres dentro de un HashSet hay que utilizar el método de inserción letra por letra, por lo cual se diseñó una función en la cual se pasa como parámetro la cadena y permitiera devolver un HashSet con los caracteres que la conforman.


```Rust
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
```

