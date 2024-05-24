import numpy as np
import pandas as pd
import os
from dotenv import load_dotenv
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression
from sklearn.metrics import mean_squared_error

from sklearn.datasets import fetch_california_housing
housing = fetch_california_housing()

load_dotenv()

from openai import OpenAI

client = OpenAI(
  organization='org-kL45XPTf2uynGJepRPWBCrP5',
  project='$PROJECT_ID',
  key={OPEN_AI_API_KEY}
)

# Load the data into a pandas DataFrame
df = pd.DataFrame(housing.data, columns=housing.feature_names)
df['PRICE'] = housing.target

# Display the first few rows of the DataFrame
print(df.head())

# Define the features and the target variable
X = df.drop('PRICE', axis=1)
y = df['PRICE']

# Split the data into training and testing sets
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Create a Linear Regression model
model = LinearRegression()

# Train the model
model.fit(X_train, y_train)

# Make predictions on the test set
y_pred = model.predict(X_test)

# Calculate the Mean Squared Error
mse = mean_squared_error(y_test, y_pred)
print(f"Mean Squared Error: {mse}")
