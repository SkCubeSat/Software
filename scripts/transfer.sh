#!/usr/bin/expect -f

# Suppress debug output
exp_internal 0
log_user 0

# ============================================================
# BeagleBone Black Serial File Transfer Script
# Usage:
#   transfer -- [options] <local-file-or-dir> [local-file-or-dir ...]
#   transfer -- -d /home/kubos/bin myfile.bin
#
# The final positional argument(s) are always local paths to transfer.
# Use -d to choose where those paths land on the OBC.
# ============================================================

set timeout 30
set serial /dev/ttyUSB1
set baud_rate 921600
set user "kubos"
set pass "Kubos123"
set remote_dir "/tmp"
set files {}
set staging_dir "/tmp/transfer_staging_[pid]"
set send_slow {1 .005}

# ---- Parse arguments ----
proc usage {{status 0}} {
    puts ""
    puts "Usage: transfer -- \[options\] <local-file-or-dir> \[local-file-or-dir ...\]"
    puts ""
    puts "The final positional argument(s) are local path(s) to transfer."
    puts "Use -d to set the destination directory on the OBC."
    puts "Options must come before local path(s); the destination is not positional."
    puts ""
    puts "Options:"
    puts "  -d <remote_dir>   Destination directory on the OBC (default: /tmp)"
    puts "  -b <baud_rate>    Serial baud rate (default: 921600)"
    puts "  -p <port>         Serial port (default: /dev/ttyUSB1)"
    puts "  -u <user>         Login user (default: kubos)"
    puts "  -v                Verbose output"
    puts "  -h                Show this help"
    puts ""
    puts "Examples:"
    puts "  transfer myfile.bin"
    puts "  transfer -- -d /home/kubos/bin myfile.bin"
    puts "  transfer -- -b 921600 -p /dev/ttyUSB1 -d /home/kubos/bin myfile.bin"
    puts "  transfer ./mydir"
    puts "  transfer file1.sh file2.sh file3.sh"
    puts ""
    exit $status
}

proc fail_usage {message} {
    puts "ERROR: $message"
    usage 1
}

proc require_option_value {args index option} {
    set next [expr {$index + 1}]
    if {$next >= [llength $args]} {
        fail_usage "$option requires a value"
    }

    set value [lindex $args $next]
    if {$value eq ""} {
        fail_usage "$option requires a non-empty value"
    }
    if {[string match "-*" $value]} {
        fail_usage "$option requires a value before the next option"
    }

    return $value
}

proc shell_quote {value} {
    return "'[string map [list ' {'\''}] $value]'"
}

proc shell_relpath {value} {
    return "./[shell_quote $value]"
}

proc send_shell {command} {
    send -s -- "$command\r"
}

proc wait_for_prompt {} {
    expect {
        timeout {
            return 1
        }
        -re {[$#] ?$} {
            return 0
        }
    }
}

proc run_remote {command description} {
    set marker "__TRANSFER_RC__[pid]__[clock seconds]"
    set marker_re $marker
    append marker_re {:([0-9]+)}
    set remote_output ""

    send_shell $command

    expect {
        timeout {
            puts "ERROR: Timed out waiting for $description"
            return 1
        }
        -re {[$#] ?$} {
            set remote_output $expect_out(buffer)
        }
    }

    send_shell "printf '\\n$marker:%s\\n' \$?"

    expect {
        timeout {
            puts "ERROR: Timed out waiting for status from $description"
            return 1
        }
        -re $marker_re {
            set rc $expect_out(1,string)
        }
    }

    if {[wait_for_prompt]} {
        puts "ERROR: Timed out waiting for prompt after $description"
        return 1
    }

    if {$rc != 0} {
        puts "ERROR: Remote $description failed with exit code $rc"
        puts "       Command: $command"
        regsub -all {\r} $remote_output "" remote_output
        set remote_output [string trim $remote_output]
        if {$remote_output ne ""} {
            puts "       Remote output:"
            foreach line [split $remote_output "\n"] {
                puts "         $line"
            }
        }
        return 1
    }

    return 0
}

if {[llength $argv] == 0} {
    usage
}

set verbose 0

# When invoked as `expect -f transfer.sh -- ...`, Expect consumes `--`.
# When invoked directly through the shebang, tolerate the same separator.
if {[llength $argv] > 0 && [lindex $argv 0] eq "--"} {
    set argv [lrange $argv 1 end]
}

set i 0
set parsing_options 1
while {$i < [llength $argv]} {
    set arg [lindex $argv $i]
    if {!$parsing_options} {
        if {[string match "-*" $arg]} {
            fail_usage "Options must come before local path(s)"
        }
        lappend files $arg
        incr i
        continue
    }

    switch -- $arg {
        "-d" {
            set remote_dir [require_option_value $argv $i $arg]
            incr i
        }
        "-b" {
            set baud_rate [require_option_value $argv $i $arg]
            incr i
        }
        "-p" {
            set serial [require_option_value $argv $i $arg]
            incr i
        }
        "-u" {
            set user [require_option_value $argv $i $arg]
            incr i
        }
        "-v" {
            set verbose 1
        }
        "-h" {
            usage
        }
        "--" {
            incr i
            while {$i < [llength $argv]} {
                lappend files [lindex $argv $i]
                incr i
            }
            break
        }
        default {
            if {[string match "-*" $arg]} {
                fail_usage "Unknown option: $arg"
            }
            set parsing_options 0
            lappend files $arg
        }
    }
    incr i
}

if {$verbose} {
    log_user 1
}

if {[llength $files] == 0} {
    fail_usage "No local file or directory specified"
}

if {![regexp {^[0-9]+$} $baud_rate] || $baud_rate <= 0} {
    fail_usage "Baud rate must be a positive integer"
}

# ---- Validate files exist ----
foreach f $files {
    if {![file exists $f]} {
        puts "ERROR: '$f' does not exist locally"
        exit 1
    }
}

# ---- Create staging directory ----
file mkdir $staging_dir

# ---- Prepare files (resolve symlinks, tar dirs, gzip everything) ----
set transfer_files {}
set remote_actions {}

foreach f $files {
    if {[file isdirectory $f]} {
        set dirname [file tail $f]
        set tarfile "$staging_dir/${dirname}.tar.gz"
        puts "📦 Packing directory: $f"
        exec tar czfh $tarfile -C [file dirname $f] $dirname
        lappend transfer_files $tarfile
        lappend remote_actions [list "tar" $dirname]
    } elseif {[file type $f] eq "link"} {
        # Resolve symlink, copy real file to staging
        set basename [file tail $f]
        set staged "$staging_dir/$basename"
        puts "🔗 Copying symlink target: $f"
        exec cp -L -- $f $staged
        set gzfile "${staged}.gz"
        exec gzip -f -- $staged
        lappend transfer_files $gzfile
        lappend remote_actions [list "gunzip" $basename]
    } else {
        set basename [file tail $f]
        set staged "$staging_dir/$basename"
        file copy -force -- $f $staged
        set gzfile "${staged}.gz"
        puts "🗜  Compressing: $f"
        exec gzip -f -- $staged
        lappend transfer_files $gzfile
        lappend remote_actions [list "gunzip" $basename]
    }
}

# ---- Check staging ----
foreach tf $transfer_files {
    if {![file exists $tf]} {
        puts "ERROR: Failed to prepare $tf"
        file delete -force -- $staging_dir
        exit 1
    }
}

# ---- Check serial port is not in use ----
set portcheck ""
if {![catch {exec fuser $serial 2>@1} portcheck_output]} {
    set portcheck [string trim $portcheck_output]
}
if {$portcheck ne ""} {
    puts "ERROR: $serial is in use by PID $portcheck"
    puts "Close minicom or other serial programs first"
    file delete -force -- $staging_dir
    exit 1
}

# ---- Connect to serial ----
puts "\n🔌 Connecting to $serial at $baud_rate baud..."
# spawn -open [open $serial w+]
# stty -F $serial raw -echo clocal -crtscts $baud_rate
if {[catch {exec stty -F $serial raw -echo clocal -crtscts $baud_rate} stty_error]} {
    puts "ERROR: Failed to configure $serial at $baud_rate baud: $stty_error"
    file delete -force -- $staging_dir
    exit 1
}
spawn -open [open $serial w+]

# ---- Login ----
send "\r"
sleep 1
send "\r"
sleep 1
send "\r"

expect {
    timeout {
        puts "ERROR: No prompt detected. Is the device powered on?"
        file delete -force -- $staging_dir
        exit 1
    }
    "ogin:" {
        send "$user\r"
        expect "assword:"
        send "$pass\r"
        if {[wait_for_prompt]} {
            puts "ERROR: No prompt detected after login"
            file delete -force -- $staging_dir
            exit 1
        }
        puts "🔑 Logged in as $user"
    }
    -re {[$#] ?$} {
        puts "✅ Already logged in"
    }
}

# ---- Ensure remote directory exists ----
set remote_dir_q [shell_quote $remote_dir]
set remote_user_q [shell_quote $user]

if {[run_remote "mkdir -p $remote_dir_q" "create destination directory"]} {
    file delete -force -- $staging_dir
    exit 1
}

# If the serial shell was already logged in as root, mkdir may create a
# root-owned destination while rz runs as kubos. /tmp should already be
# world-writable and should keep its normal root ownership.
if {$remote_dir ne "/tmp"} {
    if {[run_remote "chown $remote_user_q $remote_dir_q 2>/dev/null || true" "set destination owner"]} {
        file delete -force -- $staging_dir
        exit 1
    }
    if {[run_remote "chmod u+rwx $remote_dir_q 2>/dev/null || true" "set destination permissions"]} {
        file delete -force -- $staging_dir
        exit 1
    }
}
if {[run_remote "cd $remote_dir_q" "enter destination directory"]} {
    file delete -force -- $staging_dir
    exit 1
}

# ---- Transfer each file ----
set count 0
set total [llength $transfer_files]
set failed 0

foreach tf $transfer_files {
    set action [lindex $remote_actions $count]
    set action_type [lindex $action 0]
    set action_name [lindex $action 1]
    incr count
    set basename [file tail $tf]
    set basename_path_q [shell_relpath $basename]

    puts "\n📤 \[$count/$total\] Sending: $basename"

    if {[run_remote "rm -f $basename_path_q" "remove existing $basename"]} {
        set failed 1
        continue
    }

    send_shell "start-stop-daemon -S -a /usr/bin/rz -c $remote_user_q -- -bZ -y"

    expect {
        timeout {
            puts "ERROR: rz did not start"
            set failed 1
            continue
        }
        "waiting to receive" {}
    }

    set send_dir [file dirname $tf]
    set send_name [file tail $tf]
    set old_pwd [pwd]
    cd $send_dir
    set sz_failed [catch {exec sz -q -b -y $send_name < $serial > $serial} sz_error]
    cd $old_pwd

    if {$sz_failed} {
        puts "Note: sz returned non-zero after sending $basename; validating remote file"
        if {$sz_error ne ""} {
            puts "   sz output: $sz_error"
        }
    }

    if {[wait_for_prompt]} {
        puts "ERROR: No prompt detected after receiving $basename"
        set failed 1
        continue
    }

    # Post-transfer action
    if {$action_type eq "tar"} {
        puts "📦 Extracting: $action_name"
        if {[run_remote "tar xzf $basename_path_q && rm -f $basename_path_q" "extract $basename"]} {
            set failed 1
            continue
        }
    } elseif {$action_type eq "gunzip"} {
        puts "🗜  Decompressing: $action_name"
        if {[run_remote "gzip -df $basename_path_q" "decompress $basename"]} {
            set failed 1
            continue
        }
    }
}

# ---- Make scripts executable ----
if {[run_remote "chmod +x ./*.sh ./*.bash 2>/dev/null || true" "mark scripts executable"]} {
    set failed 1
}

# ---- Show results ----
log_user 1
puts "\n📁 Files on target in $remote_dir:"
send_shell "ls -la $remote_dir_q"
if {[wait_for_prompt]} {
    puts "ERROR: No prompt detected after listing target directory"
    set failed 1
}

# ---- Cleanup staging ----
file delete -force -- $staging_dir

if {$failed > 0} {
    puts "\n⚠  Transfer completed with errors"
    exit 1
} else {
    puts "\n✅ Transfer complete! $total file(s) sent to $remote_dir"
}
