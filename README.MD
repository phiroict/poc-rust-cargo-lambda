# PoC Cargo Lambda 

A simple example for creating a lambda from the Cargo plugin. 


# Introduction

AWS Lambda supports rust binaries there is a helper cargo plugins called `lambda`.
It will create a lambda function with the binary deployed to it.
After deployment it returns the ARN so you can use that in other 
infrastructure deploy scripts.
Note that this will:
- Version the lambdas for you after the first deploy, it matches by name 
- It will not create a VPC for you
- It will create no public interface, you need to add that yourselves.

# Tech-stack
- AWS-Vault 
- Zig
- Rust stable (>1.75)
- RustRover IDE (Or Visual Code)

## Platform 
- Windows 
- Arch Linux 

## Documentation 

[Cargo lambda plugin](https://www.cargo-lambda.info/commands/build.html)
