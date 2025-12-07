FROM python:3.11-slim

WORKDIR /app

# Create non-root user
RUN useradd -m appuser && chown -R appuser /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY main.py .

# Switch to non-root user
USER appuser

EXPOSE 8080

CMD ["python", "main.py"]
