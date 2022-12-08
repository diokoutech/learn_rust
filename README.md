# Learn _Rust_ with *Diokou Tech* 
## Compréhension 
Rust est un langage de programmation compilé et fortement typé. Il est très rapide et performant. Crée par mozila firefox en 2010.
Commandes de bases    
Cargo est le gestionnaire de paquet de rust comme npm et composer pour les autres langages.  
`cargo new nom_projet` => créer un nouveau projet rust.  
`cargo run` => lancer le projet et faire un build.  
`cargo check` => Permet de verifier le code s'il n'y a pas d'erreurs avant de générer le build.  
Le fichier cargo.toml contient toutes les dépendances du projet. il similaire a pom.xml pour celui qui vient de l'univers java et de package.json si vous venez du monde JS.
La fonction main est la fonction spéciale c'est toujour le premier code qui s'éxécute.
## Concepts
Borow checker : il va introduire des propriétés de borrowsing (prêt) et ownship (appartenance). C'est une police d'accès aux variables. 

### Déclaration variables
`let age = 25`;  
`let mut adresse = "Yeumbeul"` , cette variable est mutable (modifiante);  
NB: Par défaut toutes variables sont immutables.
## Type de données
Nous avons des types de données scalaires et composés.
### Scalaires
* Entiers
Longueur Signé Non Signé  
8 bits    i8   	  u8  
16 bits   i16     u16  
32 bits   i32     u32 
64 bits   i64     u64  
128 bits  i128    u128  
cambre    isize   usize   
## Rappels
### Api 

L’API (interface de programmation d’applications) définit la manière dont deux logiciels peuvent se connecter et communiquer entre eux via leurs points de terminaison. Par exemple, votre travail sur les API pourrait être utilisé pour rester en contact avec des parties externes (clients ou partenaires de l’entreprise). La plupart des API sont organisées autour de règles ou de normes, comme REST ou GraphQL, afin que tout le monde sache comment les utiliser.

### Microservice 

Les microservices sont des éléments de logiciel qui exécutent une tâche unique et indépendante au sein d’une application plus étendue. Ils s’opposent aux applications monolithiques car, plutôt que de construire des applications web comme une seule unité composée d’une interface utilisateur, d’une application côté serveur et d’une base de données, ils décomposent chaque partie de l’application en plusieurs circuits imprimés reliés par des API.