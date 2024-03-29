# Cirulla

Un semplice progetto per imparare a usare Rust.

L'implementazione delle regole di gioco è parziale, dal momento che lo scopo del progetto è prettamente didattico. Non è escluso che in futuro possa diventare un gioco completo.

## cirulla_lib
E' la libreria che governa le meccaniche di gioco. Al momento viene direttamente inclusa nel compilato di [cirulla_cli](#cirulla_cli), ma è destinata a venire compilata separatamente in WebAssembly per poter essere utilizzata in un futuro frontend React single player.

## cirulla_cli
Un programma a linea di comando per giocare a Cirulla. Può essere lanciato in tre modalità: **local**, **server** o **client**.

### cirulla_cli local
Viene lanciata un'istanza di gioco a linea di comando, in cui i giocatori devono alternarsi sulla postazione per effettuare la loro azione.
![Esempio di partita in locale con 4 giocatori](/assets/cirulla_cli_game.png)

### cirulla_cli server
Il programma viene lanciato in *modalità servizio* e rimane in attesa di connessioni TCP da parte dei client in rete. Lo sviluppo successivo prevede il supporto in parallelo di connessioni tramite WebSocket da parte di client su browser, in modo che possano giocare anche contro giocatori da CLI

#### Esempio di sessione TCP

La sessione inizia con la presentazione del client; il nome viene registrato e sarà mostrato agli altri giocatori. Se il nome è già registrato viene restituito errore.
```
>>> HELLO Franco
<<< HI Franco

>>> HELLO AlreadyRegistered
<<< ERROR [explanation]
```

Elenca i tavoli disponibili; viene indicato il nome, il numero di partecipanti in attesa e il numero desiderato di giocatori.
```
>>> LIST TABLES
<<< AVAILABLES
<<< something 2/4
<<< another 1/2
```

Comandi per unirsi o creare un tavolo. In caso di successo il server risponde WAIT, per poi mandare le istruzioni di gioco appena il numero di partecipanti è stato raggiunto.
```
>>> JOIN something
<<< JOINED something 
<<< WAIT

>>> JOIN unexistent
<<< ERROR [explanation]

>>> NEW TABLE friendly 4
<<< JOINED friendly 
<<< WAIT
```

### cirulla_cli client
Viene lanciata un'istanza di gioco a linea di comando che si connette a un server per iniziare una partita o unirsi a una in attesa di giocatori.