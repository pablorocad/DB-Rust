# DB-Rust
## Dudas del proyecto
-   Sintaxis
    <!-- -   [Generalidades] -->
    -   [Tipos de datos](#Tipo-de-datos)
    -   [Expresiones](#Expresiones)
    <!-- -   [Impresión] -->
    <!-- -   [Declaraciones y Asignaciones] -->
    <!-- -   [Funciones] -->
    <!-- -   [Sentencias selección] -->
    <!-- -   [Sentencias Loops] -->
    -   [Arreglos](#Arreglos)
    -   [Vectores](#Vectores)
    <!-- -   [Structs] -->
    -   [Módulos](#Modulos)
<!-- -   Simulación de bases de datos -->
<!-- -   Reporte de errores -->

## Tipo de datos

- La situación principal en la que usaría usize es al indexar algún tipo de colección (arreglo o vector).
    ````rust
    let i: usize = 0;
    let arr = [[1, 2, 3], [2, 3, 4]];
    println!("{}", arr[i][i+1]);
    ````

## Expresiones

- Los casteos explícitos siempre vendrán entre paréntesis
    ````rust
    let a: i64 = 10;
    let i: usize = (a as usize);
    ````

## Arreglos

- La definición de un arreglo se puede representar como una expresión.
    ````rust
    arr[0] = [1, 2, 3];
    arr = [ [10, 11, 12], [13, 14, 15] ];
    ```` 

## Vectores

- El acceso para vectores dentro de vectores se realiza mediante la indexación, al igual que los arreglos.
    ````rust
    let vec = vec![vec![1,2], vec![3,4]];
    println!("{}", vec[0][1]);
    ````

## Módulos

- Únicamente estarán en ambiente global.
- Solo se permite dos niveles de anidamiento.
- Un módulo es una DB.
- Un módulo anidado representa una tabla en la DB.
