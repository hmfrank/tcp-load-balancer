# Custom Load Balancing (TCP)
Dies ist die Lösung einer Programmieraufgabe.

## Aufgabe
Schreibe eine einfache Lösung, wie du TCP Load-Balancing zwischen 4 Servern verteilst.
Ziel ist es einerseits eine Lösungs-Ansatz zu beschreiben und dann der Code.

## Lösung
`cargo run` startet den Load-Balancer, der auf 127.0.0.1:8080 auf TCP-Anfragen wartet.
Er verteilt die Last an 4 lokale Server mit den (hardgecodedten) Ports 8081, 8082, 8083, 8084.
Die Server müssen separat mit `cargo run --example server 127.0.0.1:8081` (bzw. `...:8082`, usw.) gestartet werden.
Laufen die Server und der Load-Balancer, kann mit `cargo run --example tester` eine TCP-Verbindung zum Load-Balancer aufgebaut werden,
welcher dann ausgibt, welcher Client zu welchem Server zugeordnet wird.

Alternativ kann auch jedes beliebige andere Programm verwendet werden, um eine TCP-Verbindung aufzuabuen.
Bei den Servern handelt es sich um einfache Echo-Server.

Als Load-Balancing-Algorithmus wird hier round robin verwendet.
