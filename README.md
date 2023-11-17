## What’s the experiment?

1. Take N companies with different ATS
2. Calculate the lowest, average, median, and highest amount of text we need to aggregate those vacancies to become
   searchable (basically, to generate tags and skills based on that)

## Metrics to count

1. Min., Average, Median and Max. amount of text as input
2. Min., Average, Median and Max. amount of tokens as input
3. Daily, weekly, monthly refresh cost with an average update for ~3 times / month

## Results

| Filter     | Num. Companies | Num. Jobs | Min. | Max. | Avg. | Median |
|------------|----------------|-----------|------|------|------|--------|
| Characters | 5              | 361       | 1391 | 6956 | 3463 | 3567   |
| Tokens     | 5              | 361       | 283  | 1339 | 681  | 659    |

Converting to OpenAI GPT-3.5 **input tokens** pricing (0.0010$ / 1000 tokens):

| Category   | Tokens  | Price         | Price per 100 vacancies | Refresh cost (3x / month) | Refresh cost (100 vacancies / month) |
|------------|---------|---------------|-------------------------|---------------------------|--------------------------------------|
| Min.       | 283     | 0.000283$     | 0.0283$                 | 0.000849$                 | 0.0849$                              |
| Max.       | 1339    | 0.001339$     | 0.1339$                 | 0.004017$                 | 0.4017$                              |
| Avg.       | 681     | 0.000681$     | 0.0681$                 | 0.002043$                 | 0.2043$                              |
| **Median** | **659** | **0.000659$** | **0.0659$**             | **0.001977$**             | **0.1977$**                          |

Meaning that, with a budget of 100$ / month, we can approximately hold:
- Median estimate: ~50600 vacancies in the database in up-to-date state.
- Highest estimate: ~25000 vacancies in the database in up-to-date state.
