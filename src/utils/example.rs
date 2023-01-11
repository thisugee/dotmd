use indoc::indoc;

pub fn get_example_markdown() -> String {
    let value = indoc! {"
        # Example

        A third of the distance across the Beach, the meadow ends and sand begins. This slopes gradually up for another third of the distance, to the foot of the sand hills, which seem tumbled into their places by some mighty power, sometimes three tiers of them deep, sometimes two, _and sometimes only one._ A third of the distance across the Beach, the meadow ends and sand begins.

        The outline of this inner shore is most irregular, curving and bending in and out and back upon itself, making coves and points and creeks and channels, and often pushing out in flats with not water enough on them at low tide to wet your ankles.

        ## Subtitle

        This is another fine paragraph

        ### Smaller subtitle

        This is a paragraph `with` ~style~ *italic* _italic_ **bold** __bold__

        ![image](https://images.unsplash.com/photo-1673232141234-889ed2cf341c?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2456&q=80)

        *Hello [link](https://ugee.xyz/) lol*

        Hello [*link*](https://ugee.xyz/) lol \"cat\"

        Hello from *[link](https://ugee.xyz/)* to __everyone__ `reading this`

        Here's an [**important** anchor link](#example).

        line 1
        line 2

        Code & Poetry
        -------------

            You can also indent
            blocks to display
            code or poetry.

            Indented code/poetry blocks
            can be hard-wrapped.

        **Or, wrap your code in three backticks:**

        ```js
        function codeBlocks() {
          return \"Can be inserted\"
        }
        ```


        ### Embedded HTML

        With default settings, markdown-wasm allows embedded HTML.

        > It has been disabled in this demo for safety reasons, by means of setting `ParseFlags.NO_HTML`.
        > Not setting the `NO_HTML` flag allows embedding HTML like this:

        <input type=“text” value=“type”></input>


        ### Block Quotes

        > You can insert quotes by
        > preceeding each line with `>`.
        >
        > Blockquotes can also contain line
        > breaks.


        ## Lists

        ### Unordered lists

        - Unordered
        - Lists
        - Hello

        ### Ordered lists

        1. Ordered
        2. Lists
        4. Numbers are ignored
        1. Ordered

        121) Ordered lists can start
        122) with any number and
        123) use . as well as ) as a separator.

        ### Task lists

        - [ ] Task 1
        - [x] Task 2
        - [ ] Task 3
        - Regular list item

        ## Tables

        | Column 1 | Column 2 | Column 3 | Column 4
        |----------|:---------|:--------:|---------:
        | default | left | center | right

        ### Table of image file types

        | Header                    | Mime type    | Extensions | Description
        |---------------------------|--------------|------------|-------------
        | `89 50 4E 47 0D 0A 1A 0A` | image/png    | png        | PNG image
        | `47 49 46 38 39 61`       | image/gif    | gif        | GIF image
        | `FF D8 FF`                | image/jpeg   | jpg jpeg   | JPEG image
        | `4D 4D 00 2B`             | image/tiff   | tif tiff   | TIFF image
        | `42 4D`                   | image/bmp    | bmp        | Bitmap image
        | `00 00 01 00`             | image/x-icon | ico        | Icon image
    "};

    String::from(value)
}
