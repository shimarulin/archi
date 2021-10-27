pub fn get_partition_path(disk_path: &str, partition_number: &str) -> String {
    if disk_path.contains("nvme") {
        format!("{}p{}", disk_path, partition_number)
    } else {
        format!("{}{}", disk_path, partition_number)
    }
}
