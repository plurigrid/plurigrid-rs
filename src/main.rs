use std::rc::Rc;
use yrs::{
    Doc, Map, Array, Transaction, WriteTxn, Text, types::{TypeRefs}
};

pub struct Plurigrid {
    doc: Doc,
}

impl Plurigrid {
    pub fn new() -> Self {
        let mut doc = Doc::new();
        let mut txn = doc.write();

        let metadata = Map::new();
        let rows = Array::<Array<Cell>>::new();

        txn.set_map(&doc.root, "metadata", metadata);
        txn.set_array(&doc.root, "rows", rows);

        txn.commit();
        Plurigrid { doc }
    }

    pub fn add_row(&mut self, index: usize) {
        let mut txn = self.doc.write();

        let rows = txn.get_array::<Array<Cell>>(&self.doc.root, "rows");
        let new_row = Array::<Cell>::new();
        txn.insert_into_array(&rows, index, &[Rc::new(TypeRefs::Array(new_row))]);

        txn.commit();
    }

    pub fn add_cell(&mut self, row_index: usize, cell_index: usize, content: &str) {
        let mut txn = self.doc.write();

        let rows = txn.get_array::<Array<Cell>>(&self.doc.root, "rows");
        let row = txn.get_array::<Cell>(&rows, row_index);
        let new_cell = Text::from(content);
        txn.insert_into_array(&row, cell_index, &[Rc::new(TypeRefs::Text(new_cell))]);

        txn.commit();
    }

    pub fn update_cell(&mut self, row_index: usize, cell_index: usize, content: &str) {
        let mut txn = self.doc.write();

        let rows = txn.get_array::<Array<Cell>>(&self.doc.root, "rows");
        let row = txn.get_array::<Cell>(&rows, row_index);
        let cell = txn.get_text(&row, cell_index);

        txn.insert_into_text(&cell, 0, content);
        txn.delete_range_in_text(&cell, content.len()..cell.len());

        txn.commit();
    }

    pub fn delete_row(&mut self, index: usize) {
        let mut txn = self.doc.write();

        let rows = txn.get_array::<Array<Cell>>(&self.doc.root, "rows");
        txn.delete_range_in_array(&rows, index..index + 1);

        txn.commit();
    }

    pub fn delete_cell(&mut self, row_index: usize, cell_index: usize) {
        let mut txn = self.doc.write();

        let rows = txn.get_array::<Array<Cell>>(&self.doc.root, "rows");
        let row = txn.get_array::<Cell>(&rows, row_index);
        txn.delete_range_in_array(&row, cell_index..cell_index + 1);

        txn.commit();
    }
}

// Define the Cell type
pub struct Cell(Text);

impl From<&str> for Cell {
    fn from(s: &str) -> Self {
        Cell(Text::from(s))
    }
}

fn main() {
    println!("Plurigrid CRDT");
}
