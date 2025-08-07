# Ambrosia

Ambrosia attempts to construct an 'optimal' trail mix. It does this by taking a TOML file containing information about available ingredients and target quantities of different attributes (ie calories, protein, salt) and applying quadratic programming.

## Config
The program will look for this file in the route directory (parent of `src/`).

The config file has three parts:
- Basic settings
- Target information
- Item information

```toml
# config.toml

min = 0 # The minimum amount in grams of each item the mix can contain.
max = 100 # The maximum amount in grams of each item the mix can contain.
cutoff = 1 # Items with an optimal amount less than this in grams will be ignored.

# The target amount of different attributes.
# Attributes with these exact names will be found in the following items.
# Items missing attributes are assumed to have 0.
[targets]
calories = 2500
fat = 70
saturates = 20
carbohydrate = 260
sugar = 90
protein = 50
fiber = 30
salt = 6

# Items should have their attributes listed for 100g
[items.almonds]
calories = 604
fat = 51.1
saturates = 4.0
carbohydrate = 9.2
sugar = 3.0
fiber = 10.8
protein = 21.4
# salt = 0.01 # Will be assumed as 0.

# There can be as many items as you like...
```
