print("Remember to `conda activate`")
import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv('results.csv')
df['counter'].plot(kind='hist', bins=10)
plt.title(f'Distribution of counter, n={len(df)}')
plt.xlabel('counter')
plt.ylabel('Frequency')
#plt.show()
plt.savefig("counter_histogram.png")
