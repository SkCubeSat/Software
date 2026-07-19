use super::*;
use crate::mock::*;
use std::time::Duration;

#[test]
fn test_write_error() {
    let mock = MockStream::default();
    let connection = Connection::new(Box::new(mock));

    assert_eq!(
        connection
            .write(CanFrame::standard(0x123, &[1, 2]))
            .unwrap_err(),
        CanError::Timeout
    );
}

#[test]
fn test_write_good() {
    let mut mock = MockStream::default();
    let frame = CanFrame::standard(0x123, &[1, 2, 3, 4]);

    mock.write.set_input(frame.clone());
    let connection = Connection::new(Box::new(mock));

    assert_eq!(connection.write(frame), Ok(()));
}

#[test]
#[should_panic]
fn test_write_bad_input() {
    let mut mock = MockStream::default();

    mock.write.set_input(CanFrame::standard(0x123, &[1, 2]));
    let connection = Connection::new(Box::new(mock));

    let _result = connection.write(CanFrame::standard(0x124, &[1, 2]));
}

#[test]
fn test_read_good() {
    let expected = CanFrame::extended(0x0102_0304, &[0, 1, 2, 3]);
    let mut mock = MockStream::default();

    mock.read.set_output(vec![expected.clone()]);
    let connection = Connection::new(Box::new(mock));

    assert_eq!(
        connection.read(Duration::from_millis(10)).unwrap(),
        expected
    );
}

#[test]
fn test_read_frames_good() {
    let frames = vec![
        CanFrame::standard(0x100, &[0, 1]),
        CanFrame::standard(0x101, &[2, 3]),
        CanFrame::standard(0x102, &[4, 5]),
    ];
    let mut mock = MockStream::default();

    mock.read.set_output(frames.clone());
    let connection = Connection::new(Box::new(mock));

    assert_eq!(
        connection
            .read_frames(3, Duration::from_millis(10))
            .unwrap(),
        frames
    );
}

#[test]
fn test_read_payload_reassembles_matching_frames() {
    let mut mock = MockStream::default();

    mock.read.set_output(vec![
        CanFrame::standard(0x200, &[9, 9]),
        CanFrame::extended(0x0155_0401, &[0, 1, 2, 3, 4, 5, 6, 7]),
        CanFrame::extended(0x0155_0401, &[8, 9, 10, 11, 12, 13, 14, 15]),
        CanFrame::extended(0x0155_0401, &[16, 17, 18, 19]),
    ]);
    let connection = Connection::new(Box::new(mock));

    assert_eq!(
        connection
            .read_payload(
                18,
                Duration::from_millis(10),
                Some(FrameFilter::extended(0x0155_0401)),
            )
            .unwrap(),
        (0_u8..18).collect::<Vec<_>>()
    );
}

#[test]
fn test_read_payload_trims_excess() {
    let mut mock = MockStream::default();

    mock.read.set_output(vec![
        CanFrame::standard(0x123, &[0, 1, 2, 3, 4, 5, 6, 7]),
        CanFrame::standard(0x123, &[8, 9, 10, 11, 12, 13, 14, 15]),
    ]);
    let connection = Connection::new(Box::new(mock));

    assert_eq!(
        connection
            .read_payload(
                10,
                Duration::from_millis(10),
                Some(FrameFilter::standard(0x123))
            )
            .unwrap(),
        (0_u8..10).collect::<Vec<_>>()
    );
}
