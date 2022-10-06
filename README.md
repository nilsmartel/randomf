# randomf

cli to easily generate random and adjust them to your liking.

    Usage:
        random <...function>

            functions get applied one after another.

            functions are:

            floor
            round
            ceil
            between float float
            log
            pow float
            sqrt
            add float
            mul float
            sub float
            div float


## Examples
```sh
# generate random integers between 1 and 10
randomf between 1 11 floor

# generate some random number between 0 and 1
randomf

# generate some random number between 0 and 99, making higher numbers much less likely
randomf pow 2 between 0 100
```
