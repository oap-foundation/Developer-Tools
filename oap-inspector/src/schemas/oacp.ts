// Basic JSON Schemas for OACP Messages
// Based on standard Commerce/Payment patterns in OAP

export const schemas: Record<string, any> = {
    // Negotiation
    "feature-negotiation": {
        type: "object",
        properties: {
            type: { const: "https://oap.dev/schemas/negotiation" },
            features: { type: "array", items: { type: "string" } },
            threadId: { type: "string" }
        },
        required: ["type", "features"]
    },

    // Offer
    "commerce-offer": {
        type: "object",
        properties: {
            type: { const: "https://oap.dev/schemas/commerce/offer" },
            threadId: { type: "string" },
            offerer: { type: "string" }, // DID
            items: {
                type: "array",
                items: {
                    type: "object",
                    properties: {
                        name: { type: "string" },
                        description: { type: "string" },
                        price: { type: "number" },
                        currency: { type: "string", minLength: 3, maxLength: 3 },
                        sku: { type: "string" }
                    },
                    required: ["name", "price", "currency"]
                }
            },
            totalPrice: { type: "number" },
            currency: { type: "string" },
            validUntil: { type: "string", format: "date-time" }
        },
        required: ["type", "items", "totalPrice", "currency"]
    },

    // Order
    "commerce-order": {
        type: "object",
        properties: {
            type: { const: "https://oap.dev/schemas/commerce/order" },
            threadId: { type: "string" },
            offerId: { type: "string" },
            buyer: { type: "string" }, // DID
            shippingAddress: {
                type: "object",
                properties: {
                    street: { type: "string" },
                    city: { type: "string" },
                    country: { type: "string" }
                }
            }
        },
        required: ["type", "offerId"]
    },

    // Invoice
    "payment-invoice": {
        type: "object",
        properties: {
            type: { const: "https://oap.dev/schemas/payment/invoice" },
            threadId: { type: "string" },
            orderId: { type: "string" },
            amount: { type: "number" },
            currency: { type: "string" },
            paymentMethods: { type: "array" },
            destination: { type: "string" } // Account/Address
        },
        required: ["type", "amount", "currency", "destination"]
    },

    // Receipt
    "payment-receipt": {
        type: "object",
        properties: {
            type: { const: "https://oap.dev/schemas/payment/receipt" },
            threadId: { type: "string" },
            invoiceId: { type: "string" },
            transactionId: { type: "string" },
            status: { const: "completed" },
            timestamp: { type: "string", format: "date-time" }
        },
        required: ["type", "transactionId", "status"]
    }
};

export const getSchemaForType = (type: string) => {
    // Simple mapping heuristics based on type string inclusion
    if (type.includes("offer")) return schemas["commerce-offer"];
    if (type.includes("order")) return schemas["commerce-order"];
    if (type.includes("invoice")) return schemas["payment-invoice"];
    if (type.includes("receipt")) return schemas["payment-receipt"];
    if (type.includes("negotiation")) return schemas["feature-negotiation"];
    return null;
};
