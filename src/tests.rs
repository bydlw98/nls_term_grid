use super::*;

#[test]
fn test_no_cells() {
    let grid = Grid::new(1, 2, Direction::LeftToRight);
    let display = grid.fit_into_width(80).unwrap();

    assert_eq!(display.to_string(), "\n");

    let grid = Grid::new(1, 2, Direction::TopToBottom);
    let display = grid.fit_into_width(80).unwrap();

    assert_eq!(display.to_string(), "\n");
}

#[test]
fn test_one_cell() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    grid.add(DisplayCell::from(String::from("file")));
    let display = grid.fit_into_width(80).unwrap();

    assert_eq!(display.to_string(), "file\n");

    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    grid.add(DisplayCell::from(String::from("file")));
    let display = grid.fit_into_width(80).unwrap();

    assert_eq!(display.to_string(), "file\n");
}

#[test]
fn test_fit_into_width_cell_longer_than_display_width() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file11")));
    grid.add(DisplayCell::from(String::from("file111")));

    assert!(grid.fit_into_width(6).is_none());

    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file111")));
    grid.add(DisplayCell::from(String::from("file11")));

    assert!(grid.fit_into_width(6).is_none());
}

#[test]
fn test_fit_into_width_fit_into_one_line() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file2")));
    grid.add(DisplayCell::from(String::from("file3")));
    grid.add(DisplayCell::from(String::from("file4")));
    grid.add(DisplayCell::from(String::from("file5")));
    let display = grid.fit_into_width(35).unwrap();

    assert_eq!(display.to_string(), "file1  file2  file3  file4  file5\n");

    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file2")));
    grid.add(DisplayCell::from(String::from("file3")));
    grid.add(DisplayCell::from(String::from("file4")));
    grid.add(DisplayCell::from(String::from("file5")));
    let display = grid.fit_into_width(35).unwrap();

    assert_eq!(display.to_string(), "file1  file2  file3  file4  file5\n");
}

#[test]
fn test_fit_into_width_fit_into_one_line_color() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left });
    let display = grid.fit_into_width(35).unwrap();

    // if evaluated in a output device which renders ansi escape sequences
    // the following will be rendered with each cell having a color:
    // "file1  file2  file3  file4  file5\n"
    assert_eq!(display.to_string(), "\x1b[31mfile1\x1b[0m  \x1b[32mfile2\x1b[0m  \x1b[33mfile3\x1b[0m  \x1b[34mfile4\x1b[0m  \x1b[35mfile5\x1b[0m\n");

    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left });
    let display = grid.fit_into_width(35).unwrap();

    // if evaluated in a output device which renders ansi escape sequences
    // the following will be rendered with each cell having a color:
    // "file1  file2  file3  file4  file5\n"
    assert_eq!(display.to_string(), "\x1b[31mfile1\x1b[0m  \x1b[32mfile2\x1b[0m  \x1b[33mfile3\x1b[0m  \x1b[34mfile4\x1b[0m  \x1b[35mfile5\x1b[0m\n");
}

#[test]
fn test_fit_into_width_more_than_one_line_lefttoright() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    grid.add(DisplayCell::from(String::from("file10")));
    grid.add(DisplayCell::from(String::from("file20")));
    grid.add(DisplayCell::from(String::from("file3")));
    grid.add(DisplayCell::from(String::from("file400")));
    grid.add(DisplayCell::from(String::from("file5")));
    grid.add(DisplayCell::from(String::from("file100")));
    grid.add(DisplayCell::from(String::from("file2")));
    grid.add(DisplayCell::from(String::from("file30")));
    grid.add(DisplayCell::from(String::from("file4")));
    grid.add(DisplayCell::from(String::from("file500")));
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file200")));
    grid.add(DisplayCell::from(String::from("file300")));
    grid.add(DisplayCell::from(String::from("file40")));
    grid.add(DisplayCell::from(String::from("file50")));
    let display = grid.fit_into_width(35).unwrap();

    assert_eq!(
        display.to_string(),
        "file10   file20   file3   file400\n\
         file5    file100  file2   file30\n\
         file4    file500  file1   file200\n\
         file300  file40   file50\n"
    );
}

#[test]
fn test_fit_into_width_more_than_one_line_lefttoright_color() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Left});

    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Left});

    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left});
    let display = grid.fit_into_width(35).unwrap();

    // if evaluated in a output device which renders ansi escape sequences
    // the following will be rendered with each cell having a color:
    // "file10   file20   file3   file400\n\
    //  file5    file100  file2   file30\n\
    //  file4    file500  file1   file200\n\
    //  file300  file40   file50\n"
    assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m   \x1b[32mfile20\x1b[0m   \x1b[33mfile3\x1b[0m   \x1b[34mfile400\x1b[0m\n\
             \x1b[35mfile5\x1b[0m    \x1b[31mfile100\x1b[0m  \x1b[32mfile2\x1b[0m   \x1b[33mfile30\x1b[0m\n\
             \x1b[34mfile4\x1b[0m    \x1b[35mfile500\x1b[0m  \x1b[31mfile1\x1b[0m   \x1b[32mfile200\x1b[0m\n\
             \x1b[33mfile300\x1b[0m  \x1b[34mfile40\x1b[0m   \x1b[35mfile50\x1b[0m\n"
        );
}

#[test]
fn test_fit_into_width_more_than_one_line_toptobottom() {
    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    grid.add(DisplayCell::from(String::from("file10")));
    grid.add(DisplayCell::from(String::from("file20")));
    grid.add(DisplayCell::from(String::from("file3")));
    grid.add(DisplayCell::from(String::from("file400")));
    grid.add(DisplayCell::from(String::from("file5")));
    grid.add(DisplayCell::from(String::from("file100")));
    grid.add(DisplayCell::from(String::from("file2")));
    grid.add(DisplayCell::from(String::from("file30")));
    grid.add(DisplayCell::from(String::from("file4")));
    grid.add(DisplayCell::from(String::from("file500")));
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file200")));
    grid.add(DisplayCell::from(String::from("file300")));
    grid.add(DisplayCell::from(String::from("file40")));
    grid.add(DisplayCell::from(String::from("file50")));
    let display = grid.fit_into_width(35).unwrap();

    assert_eq!(
        display.to_string(),
        "file10   file5    file4    file300\n\
         file20   file100  file500  file40\n\
         file3    file2    file1    file50\n\
         file400  file30   file200\n"
    );
}

#[test]
fn test_fit_into_width_more_than_one_line_toptobottom_color() {
    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left});

    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Left});

    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Left});
    #[rustfmt::skip]
    grid.add(DisplayCell {contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left});
    let display = grid.fit_into_width(35).unwrap();

    // if evaluated in a output device which renders ansi escape sequences
    // the following will be rendered with each cell having a color:
    // "file10   file5    file4    file300\n\
    //  file20   file100  file500  file40\n\
    //  file3    file2    file1    file50\n\
    //  file400  file30   file200\n"
    assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m   \x1b[35mfile5\x1b[0m    \x1b[34mfile4\x1b[0m    \x1b[33mfile300\x1b[0m\n\
             \x1b[32mfile20\x1b[0m   \x1b[31mfile100\x1b[0m  \x1b[35mfile500\x1b[0m  \x1b[34mfile40\x1b[0m\n\
             \x1b[33mfile3\x1b[0m    \x1b[32mfile2\x1b[0m    \x1b[31mfile1\x1b[0m    \x1b[35mfile50\x1b[0m\n\
             \x1b[34mfile400\x1b[0m  \x1b[33mfile30\x1b[0m   \x1b[32mfile200\x1b[0m\n"
        );
}

#[test]
fn test_fit_into_columns_lefttoright_same_alignment() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    grid.add(DisplayCell::from(String::from("file10")));
    grid.add(DisplayCell::from(String::from("file20")));
    grid.add(DisplayCell::from(String::from("file3")));
    grid.add(DisplayCell::from(String::from("file400")));
    grid.add(DisplayCell::from(String::from("file5")));
    grid.add(DisplayCell::from(String::from("file100")));
    grid.add(DisplayCell::from(String::from("file2")));
    grid.add(DisplayCell::from(String::from("file30")));
    grid.add(DisplayCell::from(String::from("file4")));
    grid.add(DisplayCell::from(String::from("file500")));
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file200")));
    grid.add(DisplayCell::from(String::from("file300")));
    grid.add(DisplayCell::from(String::from("file40")));
    grid.add(DisplayCell::from(String::from("file50")));
    let display = grid.fit_into_columns(5);

    assert_eq!(
        display.to_string(),
        "file10   file20   file3    file400  file5\n\
         file100  file2    file30   file4    file500\n\
         file1    file200  file300  file40   file50\n"
    );
}

#[test]
fn test_fit_into_columns_toptobottom_same_alignment() {
    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    grid.add(DisplayCell::from(String::from("file10")));
    grid.add(DisplayCell::from(String::from("file20")));
    grid.add(DisplayCell::from(String::from("file3")));
    grid.add(DisplayCell::from(String::from("file400")));
    grid.add(DisplayCell::from(String::from("file5")));
    grid.add(DisplayCell::from(String::from("file100")));
    grid.add(DisplayCell::from(String::from("file2")));
    grid.add(DisplayCell::from(String::from("file30")));
    grid.add(DisplayCell::from(String::from("file4")));
    grid.add(DisplayCell::from(String::from("file500")));
    grid.add(DisplayCell::from(String::from("file1")));
    grid.add(DisplayCell::from(String::from("file200")));
    grid.add(DisplayCell::from(String::from("file300")));
    grid.add(DisplayCell::from(String::from("file40")));
    grid.add(DisplayCell::from(String::from("file50")));
    let display = grid.fit_into_columns(5);

    assert_eq!(
        display.to_string(),
        "file10  file400  file2   file500  file300\n\
         file20  file5    file30  file1    file40\n\
         file3   file100  file4   file200  file50\n"
    );
}

#[test]
fn test_fit_into_columns_lefttoright_different_alignments() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file10"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file20"), width: 6, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file3"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file400"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file5"), width: 5, alignment: Alignment::Left });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file100"), width: 7, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file2"), width: 5, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file30"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file4"), width: 5, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file500"), width: 7, alignment: Alignment::Left });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file1"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file200"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file300"), width: 7, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file40"), width: 6, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file50"), width: 6, alignment: Alignment::Left });

    let display = grid.fit_into_columns(5);

    assert_eq!(
        display.to_string(),
        "file10    file20  file3    file400  file5\n\
         file100    file2  file30     file4  file500\n\
         file1    file200  file300   file40  file50\n"
    );
}

#[test]
fn test_fit_into_columns_lefttoright_different_alignments_color() {
    let mut grid = Grid::new(1, 2, Direction::LeftToRight);
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Left });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Left });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left });

    let display = grid.fit_into_columns(5);

    // if evaluated in a output device which renders ansi escape sequences
    // the following will be rendered with each cell having a color:
    // "file10    file20  file3    file400  file5\n\
    //  file100    file2  file30     file4  file500\n\
    //  file1    file200  file300   file40  file50\n"
    assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m    \x1b[32mfile20\x1b[0m  \x1b[33mfile3\x1b[0m    \x1b[34mfile400\x1b[0m  \x1b[35mfile5\x1b[0m\n\
             \x1b[31mfile100\x1b[0m    \x1b[32mfile2\x1b[0m  \x1b[33mfile30\x1b[0m     \x1b[34mfile4\x1b[0m  \x1b[35mfile500\x1b[0m\n\
             \x1b[31mfile1\x1b[0m    \x1b[32mfile200\x1b[0m  \x1b[33mfile300\x1b[0m   \x1b[34mfile40\x1b[0m  \x1b[35mfile50\x1b[0m\n"
        );
}

#[test]
fn test_fit_into_columns_toptobottom_different_alignments() {
    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file10"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file20"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file3"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file400"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file5"), width: 5, alignment: Alignment::Right });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file100"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file2"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file30"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file4"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file500"), width: 7, alignment: Alignment::Right });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file1"), width: 5, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file200"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file300"), width: 7, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file40"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("file50"), width: 6, alignment: Alignment::Left });

    let display = grid.fit_into_columns(5);

    assert_eq!(
        display.to_string(),
        "file10  file400  file2   file500  file300\n\
         file20    file5  file30    file1  file40\n\
         file3   file100  file4   file200  file50\n"
    );
}

#[test]
fn test_fit_into_columns_toptobottom_different_alignments_color() {
    let mut grid = Grid::new(1, 2, Direction::TopToBottom);
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile10\x1b[0m"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile20\x1b[0m"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile3\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile400\x1b[0m"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile5\x1b[0m"), width: 5, alignment: Alignment::Right });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile100\x1b[0m"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile2\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile30\x1b[0m"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile4\x1b[0m"), width: 5, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile500\x1b[0m"), width: 7, alignment: Alignment::Right });

    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[31mfile1\x1b[0m"), width: 5, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[32mfile200\x1b[0m"), width: 7, alignment: Alignment::Right });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[33mfile300\x1b[0m"), width: 7, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[34mfile40\x1b[0m"), width: 6, alignment: Alignment::Left });
    #[rustfmt::skip]
    grid.add(DisplayCell { contents: String::from("\x1b[35mfile50\x1b[0m"), width: 6, alignment: Alignment::Left });

    let display = grid.fit_into_columns(5);

    // if evaluated in a output device which renders ansi escape sequences
    // the following will be rendered with each cell having a color:
    // "file10  file400  file2   file500  file300\n\
    //  file20    file5  file30    file1  file40\n\
    //  file3   file100  file4   file200  file50\n"
    assert_eq!(
            display.to_string(),
            "\x1b[31mfile10\x1b[0m  \x1b[34mfile400\x1b[0m  \x1b[32mfile2\x1b[0m   \x1b[35mfile500\x1b[0m  \x1b[33mfile300\x1b[0m\n\
             \x1b[32mfile20\x1b[0m    \x1b[35mfile5\x1b[0m  \x1b[33mfile30\x1b[0m    \x1b[31mfile1\x1b[0m  \x1b[34mfile40\x1b[0m\n\
             \x1b[33mfile3\x1b[0m   \x1b[31mfile100\x1b[0m  \x1b[34mfile4\x1b[0m   \x1b[32mfile200\x1b[0m  \x1b[35mfile50\x1b[0m\n"
        );
}
