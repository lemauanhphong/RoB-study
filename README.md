# RoB
Implementation of RøB: Ransomware over Modern Web Browsers

## Disclaimer
```
## The scripts are provided AS IS without warranty of any kind.

## We disclaim all implied warranties including, without limitation,
## any implied warranties of merchantability or of fitness for a
## particular purpose.

## The entire risk arising out of the use or performance of the
## scripts and documentation remains with you.

## In no event its authors, or anyone else involved in the creation,
## production, or delivery of the scripts be liable for any damages
## whatsoever (including, without limitation, damages for loss of
## business profits, business interruption, loss of business
## information, or other pecuniary loss) arising out of the use of
## or inability to use the sample scripts or documentation, even if
## we have been advised of the possibility of such damages.
```

## Build

Build the client:

```sh
cd ./client/
wasm-pack build -t web
```

Build the server:

```sh
cd ./server/
cargo build --release
```

## Usage

Run the server.

```bash
./server/target/release/server
```
