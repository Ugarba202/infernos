use infernos::common::types::Satoshis;
use infernos::node::lightning::backend::{LightningBackend, MockLightningBackend};

#[tokio::test]
async fn test_mock_backend_create_and_settle() {
    let backend = MockLightningBackend::new();

    // 1. Create an invoice
    let amount = Satoshis(100);
    let invoice = backend
        .create_invoice(amount, "Test invoice")
        .await
        .expect("Failed to create invoice");

    assert_eq!(invoice.amount, amount);
    assert!(!invoice.bolt11.is_empty());
    assert!(!invoice.payment_hash.0.is_empty());

    // 2. Check settlement before payment (should be false)
    let is_settled = backend
        .is_invoice_settled(&invoice.payment_hash)
        .await
        .expect("Failed to check settlement");
    assert!(!is_settled);

    // 3. Simulate payment in the mock
    backend.simulate_payment(&invoice.payment_hash).await;

    // 4. Check settlement after payment (should be true)
    let is_settled_after = backend
        .is_invoice_settled(&invoice.payment_hash)
        .await
        .expect("Failed to check settlement");
    assert!(is_settled_after);
}
