use std::io::Write;

fn main() {
    // BT1
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10, 11];

    println!("org_arr: {:?}", org_arr);
    println!("sub_arr: {:?}", sub_arr);
    if check_sub_array(&org_arr, &sub_arr) {
        println!("Mảng sub_arr là mảng con của mảng org_arr");
    } else {
        println!("Mảng sub_arr không là mảng con của mảng org_arr");
    }

    // BT2

    let org_str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let mut sub_str = String::new();
    print!("Nhập vào một vài ký tự: ");

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut sub_str).unwrap();
    sub_str.pop();
    println!("org_str: {:?}", org_str);
    println!("sub_str: {:?}", sub_str);
    
    println!("Số lượng sub_str trong org_str là: {}", substring_count(&org_str, &sub_str));
}

// Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
// let sub_arr = [6,8,10];
fn check_sub_array(arr: &[i32], sub: &[i32]) -> bool {
    let mut i = 0;
    let mut j = 0;

    let len_arr = arr.len();
    let len_sub = sub.len();

    if len_sub > len_arr {
        return false;
    }

    while i < len_arr && j < len_sub {
        if arr[i] == sub[j] {
            i += 1;
            j += 1;
            if j == len_sub {
                return true;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    return false;
}

// Bài tập 2 : Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho.
// Ví dụ: cho chuỗi str = “This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.”
// Nhập từ bàn phím : “is is” => In ra kết quả số lượng “is is” là 5

fn substring_count(org_str: &str, sub_str: &str) -> usize {
    return org_str.matches(sub_str).count();
}
