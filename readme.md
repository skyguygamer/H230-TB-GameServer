1. GameServer

A real-time MMO-style game server where player state changes are synchronized to other players.
Real time synchronized game state.
Player A changes something -> Server recieves the change -> Server updates itself -> Server tells other connected clients to update -> All other players clients update to match what happened to Player A

If Player A's health decreases 100 to 0 then they are dead, all the other players are immediently informed about that death.



3. Testing
When passing a String through to a function, that function now owns that String and is moved to its scope. This happens because if it were to copy it then you would
have multiple Strings that have the exact same pointer to the same heap in memory where the actual contents are stored. Rust needs there to only ever be one.

Integer 32's on the other hand are able to just be stored at one memory address and so copy just creates another integer where there is no need for that other pointer inside the first memory location, it's just the integer itself. In Strings the first memory address contains information about where the pointer containing the actual characters are. If you copy the String it just copys the first addresses contents which in reality still point to the exact same contents as the first String, so you now have two Strings that both point to the same heap, which is a big no no in Rust.