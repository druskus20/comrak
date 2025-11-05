use super::*;

#[test]
fn link_with_id() {
    html_opts_i(
        "[text](url){#my-id}",
        "<p><a href=\"url\" id=\"my-id\">text</a></p>\n",
        false, // disable roundtrip
        |_| {},
    );
}

#[test]
fn link_with_class() {
    html_opts_i(
        "[text](url){.my-class}",
        "<p><a href=\"url\" class=\"my-class\">text</a></p>\n",
        false, // disable roundtrip
        |_| {},
    );
}

#[test]
fn link_with_attribute() {
    html_opts_i(
        "[text](url){target=\"_blank\"}",
        "<p><a href=\"url\" target=\"_blank\">text</a></p>\n",
        false, // disable roundtrip
        |_| {},
    );
}

#[test]
fn link_with_multiple_properties() {
    html_opts_i(
        "[text](url){#my-id .my-class target=\"_blank\"}",
        "<p><a href=\"url\" id=\"my-id\" class=\"my-class\" target=\"_blank\">text</a></p>\n",
        false, // disable roundtrip
        |_| {},
    );
}

#[test]
fn image_with_properties() {
    html_opts_i(
        "![alt](url){#img-id .img-class}",
        "<p><img src=\"url\" alt=\"alt\" id=\"img-id\" class=\"img-class\" /></p>\n",
        false, // disable roundtrip
        |_| {},
    );
}

#[test]
fn link_without_properties() {
    html("[text](url)", "<p><a href=\"url\">text</a></p>\n");
}

#[test]
fn link_with_title_and_properties() {
    html_opts_i(
        "[text](url \"title\"){#my-id}",
        "<p><a href=\"url\" title=\"title\" id=\"my-id\">text</a></p>\n",
        false, // disable roundtrip
        |_| {},
    );
}
