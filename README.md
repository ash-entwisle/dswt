# Delimiter-Separated Web Tokens

I got bored and decided to make my own token format. 
This is a simple token format that uses delimiters to separate the header, payload, and hash of the token
Instead of using something like JSON (like JWT's).

## Installation

TODO

## Examples

```rust
use std::collections::HashMap;

use dswt::{
    Algorithm, 
    Token,          
    TokenManager
};

fn main() {

    // Create a payload
    let payload: HashMap<String, String> = [
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
        ("key3".to_string(), "value3".to_string()),
    ].iter().cloned().collect();

    // initialize a token manager
    let token_manager = TokenManager::new(
        Algorithm::HS256, 
        "your_key"      // set this to your secret key
    );

    // create a token from the payload
    let token: Token = token_manager.create_token(payload);

    let token_str = token.to_string();

    

}
```

## Format

The overall structure of a DSWT token is `<header>;<payload>;<hash>`. 
Each part is a base64 encoded string representing a different part of the token.
Each part is separated by a semicolon `;`.

### Header

The header is the first part of the token, 
It holds information about the token such as the version and the algorithm used to hash the payload.
It is encoded in base64, and is in the format `DSWT-<ver>/<alg>`.
Where `<ver>` is the version of the token and `<alg>` is the algorithm used to hash the payload.

### Payload

The payload is the second part of the token,
It holds the data that the token is supposed to represent.
It is encoded in base64, and is in the format `key1=value,key2=value,...`.
Each key-value pair is separated by a comma `,`.

### Hash

The hash is the last part of the token,
It is the hash of the header and payload, and is used to verify the token is valid.

### Full Token Example

To give a full example of what a DSWT token would look like, 
here is a full example token that is not base64 encoded would look like this:

```plaintext
DSWT-<ver>/<alg>;<key1>=<value>,<key2>=<value2>;<hash>
```

## Why not use JWT?

Like I said previously, this is moreso for fun and learning.
I am however using it in actual projects 
(a la [netter](https://github.com/netterapp) and [smple](https://github.com/AmmoniumStudios/smple)). 
I'm personally not a big fan of jwt's, I think they can be a lot simpler. 
