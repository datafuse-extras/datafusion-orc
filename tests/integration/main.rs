/// Tests ORC files from the official test suite (`orc/examples/`) against Arrow feather
/// expected data sourced by reading the ORC files with PyArrow and persisting as feather.
use std::fs::File;

use arrow::{compute::concat_batches, ipc::reader::FileReader, record_batch::RecordBatchReader};
use pretty_assertions::assert_eq;

use datafusion_orc::arrow_reader::ArrowReaderBuilder;

/// Checks specific `.orc` file against corresponding expected feather file
fn test_expected_file(name: &str) {
    let dir = env!("CARGO_MANIFEST_DIR");
    let orc_path = format!("{}/tests/integration/data/{}.orc", dir, name);
    let feather_path = format!(
        "{}/tests/integration/data/expected_arrow/{}.feather",
        dir, name
    );

    let f = File::open(orc_path).unwrap();
    let orc_reader = ArrowReaderBuilder::try_new(f).unwrap().build();
    let actual_schema = orc_reader.schema();
    let actual_batches = orc_reader.collect::<Result<Vec<_>, _>>().unwrap();

    let f = File::open(feather_path).unwrap();
    let feather_reader = FileReader::try_new(f, None).unwrap();
    let expected_schema = feather_reader.schema();
    let expected_batches = feather_reader.collect::<Result<Vec<_>, _>>().unwrap();

    // Gather all record batches into single one for easier comparison
    let actual_batch = concat_batches(&actual_schema, actual_batches.iter()).unwrap();
    let expected_batch = concat_batches(&expected_schema, expected_batches.iter()).unwrap();

    assert_eq!(actual_batch, expected_batch);
}

#[test]
fn column_projection() {
    test_expected_file("TestOrcFile.columnProjection");
}

#[test]
#[ignore] // TODO: nullable difference
fn empty_file() {
    test_expected_file("TestOrcFile.emptyFile");
}

#[test]
#[ignore] // TODO: Why?
fn meta_data() {
    test_expected_file("TestOrcFile.metaData");
}

#[test]
#[ignore] // TODO: error when concat record batches
fn test1() {
    test_expected_file("TestOrcFile.test1");
}

#[test]
#[ignore] // TODO: Incorrect timezone + representation differs
fn test_date_1900() {
    test_expected_file("TestOrcFile.testDate1900");
}

#[test]
#[ignore] // TODO: Incorrect timezone + representation differs
fn test_date_2038() {
    test_expected_file("TestOrcFile.testDate2038");
}

#[test]
fn test_memory_management_v11() {
    test_expected_file("TestOrcFile.testMemoryManagementV11");
}

#[test]
fn test_memory_management_v12() {
    test_expected_file("TestOrcFile.testMemoryManagementV12");
}

#[test]
fn test_predicate_pushdown() {
    test_expected_file("TestOrcFile.testPredicatePushdown");
}

#[test]
#[ignore] // TODO: Why?
fn test_seek() {
    test_expected_file("TestOrcFile.testSeek");
}

#[test]
fn test_snappy() {
    test_expected_file("TestOrcFile.testSnappy");
}

#[test]
fn test_string_and_binary_statistics() {
    test_expected_file("TestOrcFile.testStringAndBinaryStatistics");
}

#[test]
fn test_stripe_level_stats() {
    test_expected_file("TestOrcFile.testStripeLevelStats");
}

#[test]
#[ignore] // TODO: Non-struct root type are not supported yet
fn test_timestamp() {
    test_expected_file("TestOrcFile.testTimestamp");
}

#[test]
#[ignore] // TODO: Unions are not supported yet
fn test_union_and_timestamp() {
    test_expected_file("TestOrcFile.testUnionAndTimestamp");
}

#[test]
fn test_without_index() {
    test_expected_file("TestOrcFile.testWithoutIndex");
}

#[test]
fn test_lz4() {
    test_expected_file("TestVectorOrcFile.testLz4");
}

#[test]
fn test_lzo() {
    test_expected_file("TestVectorOrcFile.testLzo");
}

#[test]
fn decimal() {
    test_expected_file("decimal");
}

#[test]
fn zlib() {
    test_expected_file("demo-12-zlib");
}

#[test]
fn nulls_at_end_snappy() {
    test_expected_file("nulls-at-end-snappy");
}

#[test]
#[ignore] // TODO: Why?
fn orc_11_format() {
    test_expected_file("orc-file-11-format");
}

#[test]
fn orc_index_int_string() {
    test_expected_file("orc_index_int_string");
}

#[test]
#[ignore] // TODO: not yet implemented
fn orc_split_elim() {
    test_expected_file("orc_split_elim");
}

#[test]
fn orc_split_elim_cpp() {
    test_expected_file("orc_split_elim_cpp");
}

#[test]
fn orc_split_elim_new() {
    test_expected_file("orc_split_elim_new");
}

#[test]
#[ignore] // TODO: not yet implemented
fn over1k_bloom() {
    test_expected_file("over1k_bloom");
}
