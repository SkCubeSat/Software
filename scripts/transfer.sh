#!/usr/bin/expect -f

# Suppress debug output
exp_internal 0
log_user 0

# ============================================================
# BeagleBone Black Serial File Transfer Script
# Usage:
#   transfer myfile.bin
#   transfer -d /home/kubos/bin myfile.bin
#   transfer ./mydir
#   transfer file1.sh file2.sh file3.sh
#   transfer -d /home/kubos/bin f1 f2 f3
# ============================================================

set timeout 30
set serial /dev/ttyUSB1
set baud 921600
set user "kubos"
set pass "Kubos123"
set remote_dir "/tmp"
set files {}
set cleanup_files {}
set staging_dir "/tmp/transfer_staging_[pid]"

# ---- Parse arguments ----
proc usage {} {
    puts ""
    puts "Usage: transfer \[options\] <file1|dir1> \[file2 file3 ...\]"
    puts ""
    puts "Options:"
    puts "  -d <remote_dir>   Destination directory on target (default: /tmp)"
    puts "  -b <baud>         Baud rate (default: 921600)"
    puts "  -p <port>         Serial port (default: /dev/ttyUSB1)"
    puts "  -u <user>         Login user (default: kubos)"
    puts "  -v                Verbose output"
    puts "  -h                Show this help"
    puts ""
    puts "Examples:"
    puts "  transfer myfile.bin"
    puts "  transfer -d /home/kubos/bin myfile.bin"
    puts "  transfer ./mydir"
    puts "  transfer file1.sh file2.sh file3.sh"
    puts ""
    exit 0
}

if {[llength $argv] == 0} {
    usage
}

set verbose 0

set i 0
while {$i < [llength $argv]} {
    set arg [lindex $argv $i]
    switch -- $arg {
        "-d" {
            incr i
            set remote_dir [lindex $argv $i]
        }
        "-b" {
            incr i
            set baud [lindex $argv $i]
        }
        "-p" {
            incr i
            set serial [lindex $argv $i]
        }
        "-u" {
            incr i
            set user [lindex $argv $i]
        }
        "-v" {
            set verbose 1
        }
        "-h" {
            usage
        }
        default {
            lappend files $arg
        }
    }
    incr i
}

if {$verbose} {
    log_user 1
}

if {[llength $files] == 0} {
    puts "ERROR: No files specified"
    usage
}

# ---- Validate files exist ----
foreach f $files {
    if {![file exists $f]} {
        puts "ERROR: '$f' does not exist locally"
        exit 1
    }
}

# ---- Create staging directory ----
system "mkdir -p $staging_dir"

# ---- Prepare files (resolve symlinks, tar dirs, gzip everything) ----
set transfer_files {}
set remote_actions {}

foreach f $files {
    if {[file isdirectory $f]} {
        set dirname [file tail $f]
        set tarfile "$staging_dir/${dirname}.tar.gz"
        puts "📦 Packing directory: $f"
        system "tar czfh $tarfile -C [file dirname $f] $dirname"
        lappend transfer_files $tarfile
        lappend remote_actions [list "tar" $dirname]
    } elseif {[file type $f] eq "link"} {
        # Resolve symlink, copy real file to staging
        set realfile [file normalize $f]
        set basename [file tail $f]
        set staged "$staging_dir/$basename"
        puts "🔗 Resolving symlink: $f -> $realfile"
        system "cp -L $f $staged"
        set gzfile "${staged}.gz"
        system "gzip -f $staged"
        lappend transfer_files $gzfile
        lappend remote_actions [list "gunzip" $basename]
    } else {
        set basename [file tail $f]
        set staged "$staging_dir/$basename"
        system "cp $f $staged"
        set gzfile "${staged}.gz"
        puts "🗜  Compressing: $f"
        system "gzip -f $staged"
        lappend transfer_files $gzfile
        lappend remote_actions [list "gunzip" $basename]
    }
}

# ---- Check staging ----
foreach tf $transfer_files {
    if {![file exists $tf]} {
        puts "ERROR: Failed to prepare $tf"
        system "rm -rf $staging_dir"
        exit 1
    }
}

# ---- Check serial port is not in use ----
set portcheck [exec sh -c "fuser $serial 2>/dev/null || true"]
if {$portcheck ne ""} {
    puts "ERROR: $serial is in use by PID $portcheck"
    puts "Close minicom or other serial programs first"
    system "rm -rf $staging_dir"
    exit 1
}

# ---- Connect to serial ----
puts "\n🔌 Connecting to $serial at $baud baud..."
spawn -open [open $serial w+]
stty $baud raw -echo < $serial

# ---- Login ----
send "\r"
sleep 1
send "\r"
sleep 1
send "\r"

expect {
    timeout {
        puts "ERROR: No prompt detected. Is the device powered on?"
        system "rm -rf $staging_dir"
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
send "mkdir -p $remote_dir\r"
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

    puts "\n📤 \[$count/$total\] Sending: $basename"

    send "cd $remote_dir && start-stop-daemon -S -a /usr/bin/rz -c kubos -- -by\r"

    expect {
        timeout {
            puts "ERROR: rz did not start"
            set failed 1
            continue
        }
        "waiting to receive" {}
    }

    system "sz -b $tf < $serial > $serial"

    expect "#"

    # Post-transfer action
    if {$action_type eq "tar"} {
        puts "📦 Extracting: $action_name"
        send "cd $remote_dir && tar xzf $basename && rm $basename\r"
        expect "#"
    } elseif {$action_type eq "gunzip"} {
        puts "🗜  Decompressing: $action_name"
        send "gzip -df $remote_dir/$basename\r"
        expect "#"
    }
}

# ---- Make scripts executable ----
send "chmod +x $remote_dir/*.sh $remote_dir/*.bash 2>/dev/null\r"
expect "#"

# ---- Show results ----
log_user 1
puts "\n📁 Files on target in $remote_dir:"
send "ls -la $remote_dir/\r"
expect "#"

# ---- Cleanup staging ----
system "rm -rf $staging_dir"

if {$failed > 0} {
    puts "\n⚠  Transfer completed with errors"
    exit 1
} else {
    puts "\n✅ Transfer complete! $total file(s) sent to $remote_dir"
}
