FROM python:3.11-slim

WORKDIR /app

# Create non-root user
RUN useradd -m appuser && chown -R appuser /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY main.py .
COPY static static

# Ensure permissions for static files if needed (though read is default)
RUN chown -R appuser /app

# Switch to non-root user
USER appuser

EXPOSE 3000

CMD ["python", "main.py"]
