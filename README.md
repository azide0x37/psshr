#psshr
Parallel SSH in Rust. Inspired by Python Fabric.

```
$> psshr [push | pull | run] <command | filepath | filename> <server_json>
>>> [hostname] executing <command>
>>> [hostname] <output>
```

Arguments:	JSON with server information, command to run, or file to upload, or file to download

Operation:	Spawns tasks (threads) for each server to run command on, then collected responses and prints them in the order they were requested. 

Complexity:	O(n)
