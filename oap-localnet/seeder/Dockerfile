FROM python:3.11-slim

WORKDIR /app

# Create non-root user
RUN useradd -m appuser && chown -R appuser /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY seed.py .

# Switch to non-root user
USER appuser

CMD ["python", "seed.py"]
