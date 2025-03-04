// This tests uses PropertyValue to test static WinRT classes - those classes that lack a default interface
// and thus only provide static methods.

use test_winrt::Windows::Foundation::PropertyValue;
use windows::core::RuntimeName;

#[test]
fn static_class() -> windows::core::Result<()> {
    assert_eq!(PropertyValue::NAME, "Windows.Foundation.PropertyValue");

    // TODO: test PropertyValue's methods here

    Ok(())
}
