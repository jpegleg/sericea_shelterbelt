# Sericea Shelterbelt

_Cornus sericea_ also known as a [red dogwood](https://en.wikipedia.org/wiki/Cornus_sericea) is a flowering plant that helps shield areas from wind and has spiritual value in some cultures.

This project is a CentOS Stream 10 server that runs multiple websites or domains. It is a single VM that has domain-based routing within it, maintaining the highest levels of performance and security.

A _shelterbelt_ is [windbreak](https://en.wikipedia.org/wiki/Windbreak) is a row of plants to help reduce wind. Red dogwood is useful in some shelterbelt configurations.

This project has IaC and service templating, made in reaction to CI failures in [serotinous-cone](https://github.com/jpegleg/serotinous-cone/) and [paludification_toad](https://github.com/jpegleg/paludification_toad/).

The reason this project came into being is because `aws-lc-rs` does not support Alpine Linux or OpenBSD effectively, and because Alpine Linux K3S had some CPU utilizastion issues in recent builds.

The Paludification Toad has since overcome it's hurdles and surpased the Sericea Shelterbelt - so this project is likely already an archive.


## General flow

```
terraform
ansible
dns
pki
full checkout
```

## Service template - morphology

I have made many variations of Actix web servers in the last few years. This one is fresh featuring:

- hybrid PQC with ML-KEM via aws-lc-rs via rustls via actix via tokio
- support for serving HTML,CSS,JS statically and dynamically
- top performance
- hot reloading/deploying of PKI files (SSL certs and keys)
- hot reloading/deploying of web content (HTML, CSS, JS)
- async IO and efficient resource use

This service template is to be edited, replacing values like TEMPLATE with the name of the microservice or service name.

If you don't need to servce HTML files, then don't, replace that aspect with whatever else the service needs to do. Actix
and Tokio give us all the networking and HTTP tools we need to enable rapid and high performance web development for the internet.

Find the template in `morphology/template/` to copy into a new rust project directory to then build from.

## Design patterns

Since systemd is in play, use it. This design has a unit file for each rust app service.

The final application is where the TLS is served, no TLS termination.

All Rust stack for the networking and app logic.

HTML, CSS, and javascript easily integrated and decoupled SDLC from the rust. This way the web code
can change without needing a recompile, the web code can change with a zip file or S3 bucket, etc.

