# Intérprete de `tank`

`tank` es un pequeño lenguaje de programación, utilizado para enseñar los
fundamentos de la programación de manera sencilla y divertida. Un programa
de `tank` controla un tanque en un tablero cuadrado de 12 x 12. El tanque
puede avanzar, girar y disparar. El lenguaje tiene estructuras de control
condicionales y cíclicas.

## Ejemplo de un programa
```
// Este programa dibuja un rectángulo
// en el tablero
mientras(1 == 1){ // Ciclo infinito
  avanza; avanza;
  gira derecha;
  var x = 10;
  mientras(x != 0){
    avanza;
    x = x-1;
  }
  gira derecha;
}
```
