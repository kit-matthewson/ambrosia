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

## Output
The program will output something like this:
```
From the possible ingredients:
- almonds
- biltong
- chocolate_peanuts
- chocolate_raisins
- dried_mango
- jelly_babies
- pistachio_nuts
- puffed_rice
- salted_cashews
- unsalted_cashews

Optimal Mix:
- 90.40g almonds
- 22.30g biltong
- 59.96g chocolate_peanuts
- 17.73g chocolate_raisins
- 100.00g dried_mango
- 18.03g jelly_babies
- 100.00g puffed_rice
- 20.30g salted_cashews
- 100.00g unsalted_cashews

Total Nutrition:
- carbohydrate: 251.55 (96.75%)
- calories: 2494.48 (99.78%)
- fat: 126.63 (180.90%)
- fiber: 30.46 (101.55%)
- protein: 71.03 (142.06%)
- salt: 1.52 (25.38%)
- saturates: 23.32 (116.62%)
- sugar: 122.43 (136.03%)
Total Weight: 528.74g  
```
