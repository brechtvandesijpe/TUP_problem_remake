import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv('LBMatchDurations_matching.csv', header=None, names=['file', 'impl1', 'impl2', 'impl3'])

plt.figure(figsize=(10, 6))

plt.plot(df['impl1'], label='Hungarian', marker='o')
plt.plot(df['impl2'], label='JonkerVolgenant', marker='x')
plt.plot(df['impl3'], label='B&B 2-deep', marker='^')

plt.title('Timings of Implementations')
plt.xlabel('Test Case')
plt.ylabel('Timing (seconds)')
plt.xticks(range(len(df)), df['file'], rotation=45, ha="right")  

plt.legend()
plt.tight_layout()
plt.show()