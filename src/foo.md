Ordered alphabetically, not by importance.

- `AO3{base 10, 16, 32, 64, 1024+}` archive of our own story id
- `B{base 36, 512+}` Kindle ASIN
- `FFN{base 10, 16, 32, 64, 512+}` fan fiction dot net story id
- `N<dy base>` ISBN, for physically published books.
   The trailing check digit and (if present) leading `978`
   are omitted when encoding ISBN-10-compatible values.
   Err, rather: the value is - the minimum 10 digit value,
   to convert it into a smaller digit value where possible
   but still preserve the possibility of higher-base encodings
   any future EAN ISBN values.
- `RYL<dy ba>` royal road story id
- `WTP<dy ba>` wattpad story id
