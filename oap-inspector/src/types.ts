export interface TrafficLog {
    id: number;
    timestamp: string;
    error?: string;
    decrypted_request_body?: string;
    decrypted_response_body?: string;
    method: string;
    url: string;
    status?: number;
    request_headers: string;
    request_body?: string;
    response_headers?: string;
    response_body?: string;
    duration_ms?: number;
}
