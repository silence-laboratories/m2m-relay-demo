# Simple message relay service

- The message relay service is a simple service that relays messages between 2 MPC phones/parties. Both parties need to connect to the same relay server.
- Disclaimer: The code in this repository is not in product ready state, its for demonstration of Duo SDK in Phone-to-Phone mode.

### Quick start

- Start the message relay server.

```bash
 cargo run -p m2m-relay-svc -- --listen localhost:8000 # Replace with your own address
```
