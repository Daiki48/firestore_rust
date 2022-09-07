# Check firestore connection

Check if firestore is connected properly.

**_Wait for about 10 minutes._**

```
https://firestore.googleapis.com/v1/projects/{Project ID}/databases/(default)/documents/{collection name}
```

`Generate a new private key` is `Service acount`.

The `./key` directory is not managed by github because it contains firestore configuration information.

# Usage

```shell
$ cargo run
```
