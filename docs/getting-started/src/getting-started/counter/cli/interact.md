# Interact with server
Let's try it out!

Simply run your cli with the `read-count` command, the server address and the path to the `contract` directory as follows:
```bash
{{#include ../../../../../code/counter-cli.sh:read}}
```
You should see something like:
```
Current count is: 1
```
Your count is probably different. \
Are you one of the first people to do this tutorial or is the count already much higher?

Now let's increment the count:
```bash
{{#include ../../../../../code/counter-cli.sh:inc}}
```
And check the count again:
```bash
{{#include ../../../../../code/counter-cli.sh:read-again}}
```
If you don't see the count go up then it's probably because the solution hasn't been included in a block yet. \
Just wait a few seconds and try reading again.