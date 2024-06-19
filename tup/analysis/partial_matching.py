import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

df = pd.read_csv('pm.csv')

instances = df['INSTANCE']
pm_times = df['PM']
no_hash_times = df['NO_HASH']
no_pm_times = df['NO_PM']

n = len(df)

ind = np.arange(n)
width = 0.25

fig, ax = plt.subplots()

pm_bars = ax.bar(ind - width, pm_times, width, label='PM (memoization)')
no_hash_bars = ax.bar(ind, no_hash_times, width, label='PM (zonder memoization)')
no_pm_bars = ax.bar(ind + width, no_pm_times, width, label='zonder PM')


ax.yaxis.grid(True, color='grey', linestyle='--', linewidth=0.5, alpha=0.7)

#ax.set_axisbelow(True)

ax.set_ylabel('Rekentijd (sec)')
ax.set_title('Rekentijd per Instance en methode')
ax.set_xticks(ind)
ax.set_xticklabels(instances, rotation=45, ha="right")
ax.legend()

y_min, y_max = 0, max(max(pm_times), max(no_hash_times), max(no_pm_times))
ax.set_yticks(np.arange(y_min, y_max + 500, 500))

plt.tight_layout()
plt.savefig('execution_times.png', dpi=300)
plt.show()
