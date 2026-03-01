// Copyright (c) 2026 By David "Hankinsohl" Hankins.
// This software is licensed under the terms of the MIT License.
// Created by Hankinsohl on 1/18/2026.

#[macro_export]
macro_rules! impl_generic_table {
    ($table:ty) => {
        paste! {
            impl GenericTable for [<$table Table>] {
                fn count(&self, tx: &mut Tx) -> Result<usize, Error> {
                    let query = format!("SELECT COUNT(*) FROM {};", stringify!([<$table:snake>]));
                    let count: i64 = tx.query_row(&query, params![], |row| row.get::<usize, i64>(0))?;
                    Ok(count.try_into()?)
                }

                fn delete(&self, tx: &mut Tx) -> Result<usize, Error> {
                    let stmt = format!("DELETE FROM {};", stringify!([<$table:snake>]));
                    let count: usize = tx.execute(&stmt, ())?;
                    Ok(count)
                }

                fn drop_table(&self, tx: &mut Tx) -> Result<(), Error> {
                    let stmt = format!("DROP TABLE IF EXISTS {};", stringify!([<$table:snake>]));
                    tx.execute(&stmt, ())?;
                    Ok(())
                }

                fn name(&self) -> &str {
                    &self.name
                }

                fn partial(&self, reader: &mut dyn Read, writer: &mut dyn Write) -> Result<(), Error> {
                    let rows: Vec<[<$table Row>]> = serde_json::from_reader(reader)?;
                    let mut rng = rand::rng();
                    let divisor: i32;
                    if rows.len() < 2 {
                        divisor = 1;
                    } else {
                        divisor = rng.random_range(1..=(rows.len() as i32 / 2));
                    }
                    let filtered_rows: Vec<_> = rows
                        .into_iter()
                        .enumerate()
                        .filter(|&(index, _)| index as i32 % divisor != 0)
                        .map(|(_, value)| value)
                        .sorted()
                        .dedup()
                        .collect();
                    let serialized = JsonFormat::pretty()
                        .indent_width(Some(consts::JSON_TAB))
                        .ascii(true)
                        .format_to_string(&filtered_rows)?;
                    let mut reader = serialized.as_bytes();
                    io::copy(&mut reader, writer)?;
                    Ok(())
                }
            }
        }
    };
}
pub use impl_generic_table;
