#!/usr/bin/expect -f

# Suppress debug output
exp_internal 0
log_user 0

# ============================================================
# BeagleBone Black Serial File Transfer Script
# Usage:
#   transfer [options] <local-file-or-dir> [local-file-or-dir ...]
#   transfer -d /home/kubos/bin myfile.bin
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

# ---- Parse arguments ----
proc usage {{status 0}} {
    puts ""
    puts "Usage: transfer \[options\] <local-file-or-dir> \[local-file-or-dir ...\]"
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
    puts "  transfer -d /home/kubos/bin myfile.bin"
    puts "  transfer -b 921600 -p /dev/ttyUSB1 -d /home/kubos/bin myfile.bin"
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

if {[llength $argv] == 0} {
    usage
}

set verbose 0

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
        expect "#"
        puts "🔑 Logged in as $user"
    }
    "#" {
        puts "✅ Already logged in"
    }
}

# ---- Ensure remote directory exists ----
set remote_dir_q [shell_quote $remote_dir]
set remote_user_q [shell_quote $user]

send "mkdir -p $remote_dir_q\r"
expect "#"

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
    set basename_q [shell_quote $basename]

    puts "\n📤 \[$count/$total\] Sending: $basename"

    send "cd $remote_dir_q && start-stop-daemon -S -a /usr/bin/rz -c $remote_user_q -- -b -y\r"

    expect {
        timeout {
            puts "ERROR: rz did not start"
            set failed 1
            continue
        }
        "waiting to receive" {}
    }

    if {[catch {exec sz -b $tf < $serial > $serial} sz_error]} {
        puts "ERROR: sz failed for $tf: $sz_error"
        set failed 1
        continue
    }

    expect "#"

    # Post-transfer action
    if {$action_type eq "tar"} {
        puts "📦 Extracting: $action_name"
        send "cd $remote_dir_q && tar xzf $basename_q && rm $basename_q\r"
        expect "#"
    } elseif {$action_type eq "gunzip"} {
        puts "🗜  Decompressing: $action_name"
        send "cd $remote_dir_q && gzip -df $basename_q\r"
        expect "#"
    }
}

# ---- Make scripts executable ----
send "cd $remote_dir_q && chmod +x ./*.sh ./*.bash 2>/dev/null\r"
expect "#"

# ---- Show results ----
log_user 1
puts "\n📁 Files on target in $remote_dir:"
send "ls -la $remote_dir_q\r"
expect "#"

# ---- Cleanup staging ----
file delete -force -- $staging_dir

if {$failed > 0} {
    puts "\n⚠  Transfer completed with errors"
    exit 1
} else {
    puts "\n✅ Transfer complete! $total file(s) sent to $remote_dir"
}
