# Notas Curso de Gestión de Servidores con Rust

## Conexión TCP

##### fuente original [aquí](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) y [aquí](https://platzi.com/clases/servidores-rust/)

Tal vez has escuchado acerca de los protocolos TCP/HTTP, Hypertext Transfer Protocol (HTTP) y el Transmission Control Protocol (TCP). Ambos son protocolos de solicitud-respuesta, lo que significa que un cliente inicia solicitudes y un servidor escucha las solicitudes y proporciona una respuesta.

TCP es un protocolo de bajo nivel, que se encarga de hacer la conexión que describe los detalles de comunicación entre un servidor y otro. HTTP trabaja dentro de TCP y es el principal encargado de las solicitudes y las respuestas.

Como TCP es el encargado de manejar la conexión entre servidores, es lo primero que debemos comenzar a configurar. Dentro de la librería estándar que se encuentra en Rust, existe `std::net`, este módulo proporciona funcionalidad de red para los protocolos de control de transmisión y datagramas de usuario, así como los tipos de direcciones IP y de socket. En el módulo contamos con los métodos `TcpStream` y `TcpListener` los cuales explicaré en esta clase.

```Rust
let listener = TcpListener::bind("127.0.0.1:7373").unwrap();
```

En este caso, estamos dando valor a una variable donde por medio de la `struct` de `TcpListener` (una estructura que representa un socket server) le decimos que escuche desde Tcp conexión `127.0.0.1:7373`, para esto la función `bind` nos ayuda a decirle que esta es una nueva instancia. A diferencia de otros lenguajes, no podemos usar un puerto como 80, ya que requerimos credenciales de administrador, es por eso que nuestros puerto deben ser mayor a 1024, como este curso es para fines de aprendizaje y es un servidor de una sola tarea, lo dejaremos así por el momento. Al final tenemos `unwrap` con el cual lo único que nos indica es que termine la conexión si llegara haber errores.

Ahora trabajaremos con `TcpStream` que es una estructura que representa una secuencia Tcp entre un socket local y un socket remoto, en pocas palabras vé si hay una conexión entre cliente y servidor.

```Rust
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            println!("¡Nuevo cliente!");
        }
        Err(e) => {
            println!("Conexión fallida")
        }
    }
}
```

Analicemos el código de arriba, stream es una versión asíncrona de un iterador, `un stream` representa una conexión abierta entre cliente y servidor. Después tenemos `ìncoming` que es un iterador de `TcpListener` nos devuelve un mismo iterador que nos da una secuencia de secuencias.

Corramos nuestro código, para esto podemos usar:

```bash
cargo run
```

Esperamos en lo que termina, escribe la dirección que asignaste para la conexión dentro de `TcpListener` en nuestro caso es “127.0.0.1:7373”, dentro de tu navegador. Por el momento nos mostrará un mensaje de conexión fallida, ya que no tenemos información para mostrar. Pero no te preocupes, en tu terminal podrás ver el mensaje “¡Nuevo cliente!” cada que intentas acceder.

## Conexión HTTP

Los mensajes HTTP, son los medios por los cuales se intercambian datos entre servidores y clientes. Hay dos tipos de mensajes: peticiones, enviadas por el cliente al servidor, para pedir el inicio de una acción; y respuestas, que son la respuesta del servidor.

Agregamos una nueva librería en el proyecto

```Rust
use std::io::prelude::*;
```

En este módulo tenemos el método `prelude` que nos ayudará a leer, escribir y releer la respuesta que recibimos en bytes.

También crearemos una nueva función llamada `handle_connection` en esta nueva función, leeremos los datos del flujo TCP y los imprimimos para ver los datos que se envían desde el navegador.

Ahora nuestro código debe verse de esta manera:

```Rust
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};


fn main() {

let listener = TcpListener::bind("127.0.0.1:7373").unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {

}
```

Como puedes ver, reemplazamos el mensaje que recibíamos anteriormente para revisar que nuestra conexión funcionará, ahora solo le pasamos el “stream” a la función que acabamos de crear. Se recibe nuestro ´stream`(flujo de conexión) como un parámetro mutable, ya que no sabemos si podría leer más datos de los que solicitamos y guardarlos para después.

Dentro de nuestra función agregaremos algunas líneas extra para poder leer nuestro request y ver datos dentro de nuestro navegador, pero vamos por pasos.

Agregamos un buffer a nuestro servidor con un tamaño de 256 bytes…

```Rust
let mut buffer = [0; 256];
```

El método `stream.read`, se encargará de leer todos los bytes recibidos.

```Rust
stream.read(&mut buffer).unwrap();
```

En segundo lugar, convertimos los bytes en el buffer en una cadena y la imprimimos.

```Rust
println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
```

Si ejecutamos `cargo run` al entrar a nuestro navegador seguirá sin mostrarnos una página, pero al momento de acceder, en nuestra terminal nos arroja el siguiente mensaje, donde vemos la solicitud que enviamos acerca de nuestra información del navegador. Los resultados pueden variar ligeramente dependiendo el navegador que utilices.

Ya que obtenemos respuesta de nuestro navegador es hora de poder recibir respuesta con el estándar preferido de todos, nuestra querida respuesta 200. El código de status 200 nos indica que la respuesta ha sido exitosa, entonces ahora en lugar de recibir un print en nuestra terminal, vamos a poder recibir un status dentro de nuestro navegador.

Como saben, dentro de Rust podemos agregar `html` directo en nuestras funciones, pero esta vez queremos mantener nuestro código limpio, así que crearemos un nuevo archivo html en la carpeta raíz.

Ya que hayas creado tu interfaz, ahora es momento de agregarla a nuestro código, para esto añadiremos el módulo `std::fs`; que nos ayudará a leer el archivo como una cadena de caracteres desde nuestra función.

Eliminemos la línea `printl` de nuestro código y agreguemos las siguientes líneas:

```Rust
let contents = fs::read_to_string("index.html").unwrap();
```

A continuación, usamos `format!` para agregar el contenido del archivo como el cuerpo de la respuesta de éxito.

```Rust
let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
```

```Rust
stream.write(response.as_bytes()).unwrap(); //Nos ayuda a leer la cadena bytes que estamos recibiendo.
```

```Rust
stream.flush().unwrap(); //Esperará e impedirá que el programa continúe hasta que se escriban todos los bytes en la conexión.
```

¡Por fin! Podemos visualizar datos en nuestro navegador.

## Validación de la respuesta en el servidor

¿Qué hemos realizado hasta el momento?

1. Hemos aprendido a crear una conexión TCP.
2. Logramos enviar y leer una solicitud HTTP.
3. Logramos tener una vista al momento que nuestro servidor está activo.
4. Hemos aprendido a enviar status 200 por el navegador.

Para tener nuestro servidor completo, necesitamos otro status que podría ayudarnos a validar si no obtenemos respuesta a nuestra solicitud. El tan temido `404 Not Found`. Para esto ya sabemos como leer archivos html dentro de nuestra función principal, ahora debemos crear una nueva interfaz mostrando mensaje de error.

Haremos una modificación a nuestro código, esta vez usaremos `ìf` y `else` para manejar nuestras dos peticiones. Al código que ya hemos venido trabajando, nos posicionamos en nuestra función `handle_connection`, ahí es donde crearemos nuestras condiciones.

```Rust
let get = b"GET / HTTP/1.1\r\n";
```

Primero, agregamos la respuesta de nuestra solicitud en la una variable llamada `get`. Debido a que estamos leyendo bytes sin procesar en el `buffer`, transformamos `get` en una cadena de bytes agregando la sintaxis de cadena de bytes “b” al comienzo de la respuesta.

Ahora sí, comenzamos con nuestra condición:

```Rust
if buffer.starts_with(get) {}
```

Esto significa que si `buffer` recibe una respuesta en `get` y nuestra condición se cumple, quiere decir que tendremos que arrojar nuestro mensaje con el status `200` de petición exitosa.

```Rust
let contents = fs::read_to_string("hello.html").unwrap();
let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
stream.write(response.as_bytes()).unwrap();
stream.flush().unwrap();
```

Pero en caso de que esta condición no se llegara a cumplir, es decir, que se está enviando otro tipo de petición y no nuestra dirección, por ejemplo: `127.0.0.1:7373/algo´ esto debería arrojarnos un mensaje de error en nuestro navegador.

Para esto, primero crearemos un archivo html que nos muestre un mensaje de error en mi caso es `404.html`.

```Rust
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hola</title>
  </head>
  <body>
    <h1 style="text-align:center;">¡Oops!</h1>
    <h1 style="text-align:center;">Lamento el inconveniente, pero no encuentro la página que solicitas</h1>
    <img src="https://www.rust-lang.org/logos/error.png" style="display:block;margin:auto;" width="600" height="500"/>
  </body>
</html>
```

Ya que tenemos nuestro mensaje de error para mostrar en el navegador, creamos nuestra respuesta en nuestra condicionante:

```Rust
} else {
    let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let contents = fs::read_to_string("404.html").unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

Si revisas, lo único que llega a cambiar es nuestra variable `response` que ahora es `status_line` donde recibe mensaje de error a la petición, y ahora leemos nuestro archivo `404.html` en lugar de nuestro archivo de bienvenida.

Es hora de que ejecutes `cargo run` y veas los resultados.

Ya tenemos un servidor que puede enviar y recibir una sola petición.

## Uso de concurrencia en Rust

Antes de pasar a crear un servidor que soporte múltiples peticiones, daremos un repaso rápido al manejo de concurrencia dentro de Rust y la creación de hilos.

La programación concurrente, donde diferentes partes de un programa se ejecutan de manera independiente, y la programación paralela , donde diferentes partes de un programa se ejecutan al mismo tiempo, son cada vez más importantes a medida que más computadoras aprovechan sus múltiples procesadores.

Rust pretende ser un lenguaje con muy buen soporte para la concurrencia y paralelismo flexible, que permitan poderosas API sin perder ninguna garantía de seguridad de hilos (o seguridad de memoria).

Los errores de seguridad de la memoria y los errores de concurrencia a menudo se reducen al código de acceso a los datos cuando no debería. El arma secreta de Rust es la [ownership (propiedad)](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), una disciplina para el control de acceso que los programadores de sistemas intentan seguir, pero que el compilador de Rust comprueba estáticamente por ti.

Esto significa que puede elegir entre una amplia variedad de paradigmas (paso de mensajes, estado compartido, sin bloqueo, puramente funcional), y Rust lo ayudará a evitar dificultades comunes.

Algunos ejemplos de concurrencia en Rust:

1. Los canales de Rust imponen el aislamiento del hilo.
2. “Bloquear datos, no código” se aplica en Rust.
3. La seguridad de los hilos no es solo documentación; es ley.
4. Incluso las formas más atrevidas de compartir marcos de pila entre subprocesos están garantizadas con seguridad en Rust.

Todos estos beneficios provienen del modelo de ownership (propiedad) de Rust y, de hecho, los bloqueos, canales, estructuras de datos sin bloqueo, etc. se definen en bibliotecas, no en el lenguaje principal. Eso significa que el enfoque de concurrencia de Rust es abierto.

La programación concurrente viene en muchos estilos, pero uno particularmente simple es el paso de mensajes, donde los hilos o actores se comunican enviando mensajes entre sí.

## Creación de hilos con Rust

En la clase anterior vimos cómo se maneja la concurrencia en Rust, abordamos que para mejor manejo de errores es dividir el código en multiprocesos o hilos. En esta clase veremos cómo podemos crear hilos, moverlos y la comunicación entre ellos.

### Creando un hilo (thread)

Para comenzar a crear un hilo, comenzaremos nuestro código invocando el módulo
`std::thread`, nos permitirá usar todos sus métodos y funciones que nos facilitan la creación de hilos, veamos un ejemplo.

```Rust
use std::thread;

fn main() {
    let example = thread::spawn(move || {
        println!("Hola desde tu primer hilo");
    });
    let _ = example.join();
}
```

Este ejemplo lo habrás visto en temas más básicos de Rust, pero expliquemos un poco de qué es lo que está pasando.

Primero creamos una variable llamada `example` donde nos permite traer la función `thread::spawn`. Spawn es una función genérica con un argumento y dos parámetros de tipo:

```Rust
fn spawn<F, T>(f: F) -> JoinHandle<T>
```

1. El tipo F puede ser cualquier función / cierre que devuelva un T ( FnOnce() -> T), se puede transferir de forma segura entre subprocesos (Send) y no contiene referencias de corta duración ( 'static).
2. El tipo T puede ser de cualquier tipo, siempre que se pueda transferir entre subprocesos y no contenga referencias de corta duración.

El JoinHandle\<T> permite recuperar el T que f regresa, a través de su método `join`.

El `move` es un cierre de hilos que a menudo se usa junto a thread::spawn porque le permite usar datos de un hilo en otro hilo.

También puedes agregar:

```Rust
thread::sleep_ms(1000);
```

Las llamadas `thread::sleep` obligan a un subproceso a detener su ejecución durante un período breve, lo que permite que se ejecute un subproceso diferente. Los subprocesos probablemente se turnen, pero eso no está garantizado: depende de cómo su sistema operativo programe los subprocesos.

Veamos un segundo caso:

```Rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hola {} desde el primer hilo", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hola {} desde el segundo hilo", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    for i in 1..4 {
        println!("hola {} desde el tercer hilo", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

El ejemplo imprime un texto de un hilo principal y otro texto de un hilo nuevo. Ten en cuenta que con esta función, el nuevo subproceso se detendrá cuando finalice el subproceso principal, haya terminado o no de ejecutarse. El resultado de este programa puede ser un poco diferente cada vez.

En este segundo caso agregamos el método `Duration` qué es un tipo de duración para representar un lapso de tiempo, generalmente utilizado para tiempos de espera del sistema.

## Traits, Metodos y Mutex para concurrencia

Una herramienta importante que Rust tiene para lograr la concurrencia de envío de mensajes es el `channel`, un concepto de programación que contiene la biblioteca estándar de Rust.

Rust pretende ser un lenguaje con muy buen soporte para la concurrencia; y tiene tres partes: `ownership & lifetimes`, los traits `Send y Sync`, y `Mutex`.

### Channel

Trabajaremos en un programa que tiene un hilo para generar valores y enviarlos por un canal, y otro hilo que recibirá los valores y los imprimirá. Enviaremos valores simples entre hilos usando un canal para ilustrar la función.

```Rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hola"),
            String::from("desde"),
            String::from("el"),
            String::from("hilo 1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("vemos"),
            String::from("como "),
            String::from("trabaja"),
            String::from("hilo 2"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Obetener mensaje {}", received);
    }
}
```

Analicemos un poco lo que hacemos:

```Rust
use std::sync::mpsc;
```

Crea un nuevo canal asincrónico, devolviendo las mitades del transmisor / receptor. Todos los datos enviados en el remitente estarán disponibles en el receptor en el mismo orden en que fueron enviados, y ningún envío bloqueará el hilo de llamada. Aquí tx funcionará como transmisor de datos y rx como receptor.

```Rust
let (tx, rx) = mpsc::channel();
```

Vamos a usar `mpsc` y expandir el código, esta vez, antes de crear el primer hilo generado, llamamos clone al extremo emisor del canal. Esto nos dará un nuevo controlador de envío que podemos pasar al primer subproceso generado. Pasamos el extremo del envío original del canal a un segundo hilo generado. Esto nos da dos hilos, cada uno enviando mensajes diferentes al extremo receptor del canal.

```Rust
let tx1 = mpsc::Sender::clone(&tx);
```

Eso logrará que tengamos una salida similar a esta:

### Send & Sync

Los `traits` `Send` y `Sync` capturan y controlan las dos formas más comunes en las que se accede a un dato y se lo arroja a los hilos, dictando si es seguro transferir la propiedad o pasar una referencia a otro hilo.

Los traits son “marker traits”, lo que significa que no tienen métodos y no proporcionan inherentemente ninguna funcionalidad, solo son útiles para hacer cumplir invariantes relacionados con la concurrencia.

Con `Send`, indica que la propiedad del tipo que implementa el envío se puede transferir entre subprocesos.

El segundo de estos traits es llamado `Sync`. le indica al compilador que algo de este tipo no tiene posibilidad de introducir inseguridad en memoria cuando es usado de manera concurrente por múltiples hilos de ejecución.

### Mutex

Para permitir que los hilos muten datos compartidos, necesitamos un tipo que pueda forzar el acceso mutuo exclusivo a los datos compartidos en tiempo de ejecución. La biblioteca estándar de Rust proporciona `std :: sync :: Mutex \<T>` para este propósito.

Mutex es una abreviatura de exclusión mutua , ya que, en un mutex, solo un hilo puede acceder a algunos datos en un momento dado. Para acceder a los datos en un mutex, un subproceso primero debe indicar que desea acceder al solicitar la adquisición del bloqueo del mutex . El bloqueo es una estructura de datos que forma parte del mutex que realiza un seguimiento de quién tiene actualmente acceso exclusivo a los datos.

Los mutexes tienen fama de ser difíciles de usar porque debes recordar dos reglas:

1. Debe intentar adquirir el bloqueo antes de usar los datos.
2. Cuando haya terminado con los datos que guarda el mutex, debe desbloquear los datos para que otros hilos puedan adquirir el bloqueo.

## Mejora de nuestro servidor web con multitareas: ThreadPool

Si se preguntaba porque primero pasamos por el tema sobre concurrencia, es porque es hora de que la pongamos en acción. Para crear un servidor que nos acepte varias peticiones, hacemos el uso de la concurrencia. Es por eso que es importante que hayas practicado y entendido los conceptos que manejamos anteriormente, si no es así, puedes darle un repaso antes de continuar.

Aquí regresaremos a nuestro código donde creamos un servidor de una sola petición, le haremos algunas mejoras para que pueda aceptar múltiples peticiones a la vez, por eso trabajaremos en nuestro ambiente local, para poder ver las respuestas en nuestro navegador.

Para este ejercicio necesitamos reestructurar las carpetas de nuestro proyecto, para esto crearemos una nueva carpeta llamada `bin` dentro de nuestra carpeta `src`, moveremos nuestro archivo principal main.rs dentro de la carpeta `bin`. Dentro de nuestra carpeta crearemos un nuevo archivo llamado `lib.rs`.

Debido a que no estamos usando dependencias dentro de nuestro proyecto, no podemos usar threadpool por parte crates.io, entonces debemos crear nuestra propia librería de `Threadpool`, el cual será llamado por medio de `use nombredetuproyecto::ThreadPool;`, en nuestro archivo `main`.

Comencemos con nuestro archivo `main.rs`. En nuestra función principal pasamos el stream a la función `handle_connection`, ahora queremos que nuestro grupo de subprocesos funcione de manera similar y familiar, por lo que cambiar de subprocesos a un grupo de subprocesos es fácil usando `ThreadPool::new` para crear un nuevo grupo de hilos con un número configurable de hilos.

En nuestra función principal agregaremos:

```Rust
let pool = ThreadPool::new(3);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
```

Utilizamos `ThreadPool::new` para crear un nuevo grupo de subprocesos con un número configurable de subprocesos, en este caso 3. Luego, en el bucle `for`, `pool.execute` tiene una interfaz similar `thread::spawn` a la que requiere un cierre que el grupo debe ejecutar para cada flujo. Necesitamos implementarlo pool.execute para que tome el cierre y lo entregue a un subproceso en el grupo para que se ejecute.

Agregamos un bloque else if después del if bloque para verificar la solicitud de /sleep. Cuando se recibe esa solicitud, el servidor dormirá durante 5 segundos antes de mostrar la página HTML correcta.

```Rust
let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
```

Creamos una nueva variable llamada `sleep`, agregamos un bloque else if después del if bloque para verificar la solicitud de `sleep`. Cuando se recibe esa solicitud, el servidor dormirá durante 5 segundos antes de mostrar la página HTML correcta. Para eso usamos `use std::time::Duration`; qué vimos en nuestro capítulo de concurrencia y lo agregamos en la parte inicial de nuestro código, donde llamamos a nuestras librerías.

Ahora tenemos configurado nuestro archivo main.rs, si lo ejecutamos, este dará un mensaje de error que nos dice que necesitamos un ThreadPool, ya que hemos creado un grupo de subprocesos pero no tenemos ninguna dependencia de donde podamos llamarla. Nuestra implementación de ThreadPool será independiente del tipo de trabajo que nuestro servidor web está haciendo, usaremos el archivo que creamos llamado `lib.rs`.

Esta es la definición más simple de una estructura ThreadPool que podemos tener por ahora. No vamos a compilar el código todavía, ya que aún nos falta definir el comportamientos de la implementación Threadpool, pero eso lo veremos en nuestra siguiente clase.

## Configurando nuestro Threadpool

Aplicaremos algunas funciones a nuestro `Threadpool` vamos a ir explicando uno por uno, al final tú deberías terminar con tu implementación de la misma manera.

A continuación necesitamos crear una función asociada nombrada `new` para `ThreadPool`. También sabemos que `new` debe tener un parámetro que pueda aceptar 4 como argumento y debe devolver una instancia ThreadPool Implementemos la función `new` más simple que tendrá esas características:

```Rust
pub struct ThreadPool;

impl ThreadPool {
   pub fn new(size: usize) -> ThreadPool {
    ThreadPool
   }
}
```

Definiremos el método `execute` ThreadPool para tomar un cierre como parámetro. Vemos que podemos tomar como parámetros cierres con tres rasgos diferentes: Fn, FnMut, y FnOnce. Necesitamos decidir qué tipo de cierre usar aquí. Sabemos que terminaremos haciendo algo similar al método `thread::spawn` de la biblioteca estándar, por lo que podemos ver qué límites tiene la firma de thread::spawn en su parámetro.

```Rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
   where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
```

Cambiamos la definición de ThreadPoolmantener un vector de `thread::JoinHandle<()>instancias`, una vez que se recibe un tamaño válido, nuestro ThreadPool crea un nuevo vector que puede contener `size` cierto tamaño de elementos. Para otros fines donde no conoces el tamaño, puedes usar `with_capacity` que preasignar el tamaño dentro del vector.

```Rust
pub struct ThreadPool {
   threads: Vec<thread::JoinHandle<()>>,
}
impl ThreadPool {
pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut threads = Vec::with_capacity(size);

    for _ in 0..size {
    }
    ThreadPool { threads }
   }
```

Cambiaremos el nombre del campo ThreadPool de threads a workers porque ahora contendrá instancias Worker en lugar de instancias JoinHandle<()>. Como no es necesario que `main` sepa cómo funcionan los worker, en este caso serán privadas. Necesitamos que se comuniquen entre hilos, es por eso que aplicaremos `channel` en este ejemplo.

El ThreadPool creará un canal y se mantendrá en el lado emisor, cada Worker se aferrara al lado receptor, crearemos una nueva estructura llamada `Job` que contendrá los cierres que queremos enviar por el canal.

El método `execute` enviará el trabajo que desea ejecutar en el lado de envío del canal. En su hilo, el bucle Worker pasará por su lado receptor del canal y ejecutará los cierres de cualquier trabajo que reciba.

Pero aquí tendremos un problema, la implementación del canal que proporciona Rust es de múltiples productores, un solo consumidor. Esto significa que no podemos simplemente clonar el extremo consumidor del canal para arreglar este código.

Para solucionar la parte de compartir la propiedad entre múltiples subprocesos y permitir que los subprocesos muten el valor, debemos usar Arc\<Mutex\<T>>. El Arc permitirá que varios worker posean el receptor y Mutex garantizará que solo un worker obtenga un trabajo del receptor a la vez.

Finalmente implementamos el método `execute`. También cambiaremos `Job` de una estructura a un alias de tipo para un objeto `trait` que contenga el tipo de cierre que `execute` recibe.

Después de crear una nueva instancia `Job` con el cierre que ingresamos execute, enviamos ese trabajo al final del canal de envío. Estaremos haciendo un llamado `unwrap` en caso de que el envío fracase.

Nuestra biblioteca de librerías en lib.rs debe lucir de esta manera:

```Rust
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
```

Por otro lado, como solo estoy llamando a Threadpool en nuestro archivo main, pero esta es una librería que creamos, como tal aún no se encuentra dentro de la biblioteca de Rust, es por eso que debemos llamarlo de la siguiente manera:

```Rust
extern crate nombredetuproyecto;
```

## Uso de Drop

Ahora que estamos un poco más familiarizados con el uso de `traits` hablemos de uno en particular que nos ayudará a poder limpiar nuestro servidor después de terminar de hacer cada petición.

Por el momento tenemos un proyecto que cuando lo corremos nos da un servidor multiproceso, también obtenemos algunas advertencias sobre el workers, id y thread, que no estamos utilizando de una manera directa que nos recuerda que no estamos limpiando nada. Al momento que queremos apagarlo, usamos la técnica que nunca falla Ctrl + C en nuestra terminal, eso hace que apaguemos nuestro servidor, por consecuencia, termina todo los subprocesos que se encuentran activos.

En este tipo de casos es cuando podríamos usar el `trait` `Drop`, o mejor conocido como Destructor, solo tiene un método: `drop`, que se llama automáticamente cuando un objeto sale del alcance. El uso principal del rasgo Drop es liberar los recursos que posee la instancia.

Solo hay dos métodos que se pueden usar por `Drop`:

```Rust
fn drop(&mut self)
```

Este método es llamado principalmente cuando el valor está fuera del scope.

Y por otro lado, tenemos el método `Panic`. que automáticamente destruye el proceso al momento de ser llamado.

Dentro de nuestro proyecto, ¿por qué se podría implementar `Drop`? Ya que generalmente se trabaja con `structs`, dentro de nuestro proyecto tenemos un tamaño y contador de procesos, podemos hacer uso de Drop, para hacer que ese contador baje a cero, cuando esto suceda, pueda apagar y limpiar nuestro proceso. Esto haría que nuestro servidor se apague y se limpie de una manera elegante.

## Uso de frameworks y crates

En este curso lo que hicimos, aprendimos lo suficiente que tu deberías saber para poder crear tu propio servidor de manera manual, conociste cómo trabajan los hilos, hablamos de concurrencia que es un aspecto importante para la creación de estos y creamos nuestro servidor sencillo. Usamos el código general educativo que hay en Rust pero de una forma mejor explicada.

La manera en que nosotros hicimos nuestro servidor puede ayudarte para crear sistemas desde muy bajo nivel, como tu propio sistema operativo, librerías, métodos. Por ahora, a estas alturas ya hay varios frameworks y dependencias que nos ayudan a crear nuestro servidor para un desarrollo web de una manera más sencilla.

Por ejemplo tenemos a [Rocket](https://rocket.rs/), que es uno de los frameworks más completos y preferidos por los desarrolladores, su principal punto fuerte es que no te hace sacrificar la velocidad por un entorno productivo y viceversa, ayuda a escribir aplicaciones web rápidas, seguras y sin sacrificar la flexibilidad, la usabilidad o la seguridad de tipos.

También tenemos a [Actix](https://actix.rs/), es muy adecuado para escribir servicios con lógica y componentes duros. También proporciona muchas funciones (logging, http/2, etc.) listas para usar.

[Nickel](https://nickel-org.github.io/) es un framework para crear aplicaciones web renderizadas por el servidor. Su API está inspirada en el popular framework Express para JavaScript. Nickel facilita el mapeo de datos JSON directamente en su struct, y de forma predeterminada, Nickel detecta todos los errores con su valor predeterminado ErrorHandler e intenta tomar medidas razonables. Por lo tanto, no es necesario escribir su propio errorHandler personalizado.

Por último, pero no menos importante, tenemos a [Yew](https://docs.rs/yew/0.2.0/yew/), inspirado por React , Yew es un framework para construir aplicaciones de cliente web multiproceso con WebAssembly comúnmente conocido como WASM.

No nos olvidemos [crates.io](https://crates.io/). El cual es una lista enorme de dependencias que podemos usar dentro de nuestro proyecto. En el caso de usar dependencias, si podemos cambiar de `cargo run` a `cargo build` para correr nuestras aplicaciones.