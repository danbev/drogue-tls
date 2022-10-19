#!/usr/bin/python3

from securesystemslib import interface
from in_toto.models.layout import Layout
from in_toto.models.metadata import Metablock

def main():
  key_sscs_private = interface.import_rsa_privatekey_from_file("artifacts/sscs-tool")
  key_sscs_public = interface.import_rsa_publickey_from_file("artifacts/sscs-tool.pub")

  layout = Layout.read({
      "_type": "layout",
      "keys": {
          key_sscs_public["keyid"]: key_sscs_public,
      },
      "steps": [{
          "name": "clone_project",
          "expected_materials": [],
          "expected_products": [
              #["CREATE", "embedded-tls/src/lib.rs"],
              ["CREATE", "embedded-tls"],
              ["ALLOW", "embedded-tls/*"],
          ],
          "pubkeys": [key_sscs_public["keyid"]],
          "expected_command": [
              "git",
              "clone",
              "https://github.com/drogue-iot/embedded-tls.git"
          ],
          "threshold": 1,
        },{
          "name": "update-version",
          "expected_materials": [
              ["MATCH", "embedded-tls/*", "WITH", "PRODUCTS", "FROM", "clone_project"],
              ["ALLOW", "Cargo.toml"],
              ["DISALLOW", "*"],
          ],
          "expected_products": [
              ["MODIFY", "Cargo.toml"],
              ["ALLOW", "Cargo.lock"],
              ["ALLOW", "sscs-tool.pub"],
              ["ALLOW", "sscs-tool"],
              ["ALLOW", "root.layout"],
              ["DISALLOW", "*"]],
          "pubkeys": [key_sscs_public["keyid"]],
          "expected_command": [],
          "threshold": 1,
        }],
      "inspect": [{
          "name": "cargo-fetch",
          "expected_materials": [
              ["MATCH", "embedded-tls/*", "WITH", "PRODUCTS", "FROM", "clone_project"],
              ["ALLOW", "embedded-tls/target"],
              ["ALLOW", "sscs-tool.pub"],
              ["ALLOW", "sscs-tool"],
              ["ALLOW", "root.layout"],
              ["DISALLOW", "*"],
          ],
          "expected_products": [
              ["MATCH", "embedded-tls/Cargo.toml", "WITH", "PRODUCTS", "FROM", "update-version"],
              ["MATCH", "*", "WITH", "PRODUCTS", "FROM", "clone_project"],
              ["ALLOW", "embedded-tls/target"],
              ["ALLOW", "sscs-tool.pub"],
              ["ALLOW", "sscs-tool"],
              ["ALLOW", "root-layout"],
          ],
          "run": [
              "git",
              "clone",
              "https://github.com/drogue-iot/embedded-tls.git"
          ],
        }],
  })

  metadata = Metablock(signed=layout)

  print("Created artifacts/root.layout file")
  metadata.sign(key_sscs_private)
  metadata.dump("artifacts/root.layout")

if __name__ == '__main__':
  main()
