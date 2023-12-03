from __future__ import print_function
import os, sys, re
import argparse, time
import atexit

from typing import Any, Iterable, Sequence
from subprocess import Popen, STDOUT, PIPE
from select import select

debug_file = None
log_file = None


def debug(data: Any):
    if debug_file:
        debug_file.write(data)
        debug_file.flush()


def log(data: Any, end: str = "\n"):
    if log_file:
        log_file.write(data + end)
        log_file.flush()
    print(data, end=end)
    sys.stdout.flush()


sep = "\n"
rundir = None

parser = argparse.ArgumentParser(
    description="Run a test file against a Mal implementation"
)
parser.add_argument("--rundir", help="change to the directory before running tests")
parser.add_argument(
    "--start-timeout", default=10, type=int, help="default timeout for initial prompt"
)
parser.add_argument(
    "--test-timeout",
    default=20,
    type=int,
    help="default timeout for each individual test action",
)
parser.add_argument(
    "--pre-eval",
    default=None,
    type=str,
    help="Mal code to evaluate prior to running the test",
)
parser.add_argument(
    "--log-file",
    type=str,
    help="Write messages to the named file in addition the screen",
)
parser.add_argument(
    "--debug-file", type=str, help="Write all test interaction the named file"
)
parser.add_argument(
    "--hard",
    action="store_true",
    help="Turn soft tests (soft, deferrable, optional) into hard failures",
)

# Control whether deferrable and optional tests are executed
parser.add_argument(
    "--deferrable",
    dest="deferrable",
    action="store_true",
    help="Enable deferrable tests that follow a ';>>> deferrable=True'",
)
parser.add_argument(
    "--no-deferrable",
    dest="deferrable",
    action="store_false",
    help="Disable deferrable tests that follow a ';>>> deferrable=True'",
)
parser.set_defaults(deferrable=True)
parser.add_argument(
    "--optional",
    dest="optional",
    action="store_true",
    help="Enable optional tests that follow a ';>>> optional=True'",
)
parser.add_argument(
    "--no-optional",
    dest="optional",
    action="store_false",
    help="Disable optional tests that follow a ';>>> optional=True'",
)
parser.set_defaults(optional=True)

parser.add_argument(
    "test_file", type=str, help="a test file formatted as with mal test data"
)
parser.add_argument(
    "mal_cmd",
    nargs="*",
    help="Mal implementation command line. Use '--' to "
    "specify a Mal command line with dashed options.",
)
parser.add_argument(
    "--crlf",
    dest="crlf",
    action="store_true",
    help="Write \\r\\n instead of \\n to the input",
)


class Runner:
    def __init__(self, args: str | Sequence[str], line_break: str = "\n"):
        # Cleanup child process on exit
        atexit.register(self.cleanup)

        env = os.environ
        env["TERM"] = "dumb"
        env["INPUTRC"] = "/dev/null"
        env["PERL_RL"] = "false"
        self.p = Popen(
            args,
            bufsize=0,
            stdin=PIPE,
            stdout=PIPE,
            stderr=STDOUT,
            env=env,
        )
        self.stdin = self.p.stdin
        self.stdout = self.p.stdout

        # print "started"
        self.buf = ""
        self.last_prompt = ""

        self.line_break = line_break

    def read_to_prompt(self, prompts: Iterable[str], timeout: float):
        end_time = time.time() + timeout
        while time.time() < end_time:
            [outs, _, _] = select([self.stdout], [], [], 1)
            if (stdout := self.stdout) in outs and stdout is not None:
                new_data = stdout.read(1).decode("utf-8")
                debug(new_data)
                # Perform newline cleanup
                self.buf += new_data.replace("\r", "")
                for prompt in prompts:
                    regexp = re.compile(prompt)
                    match = regexp.search(self.buf)
                    if match:
                        end = match.end()
                        buf = self.buf[0 : match.start()]
                        self.buf = self.buf[end:]
                        self.last_prompt = prompt
                        return buf
        return None

    def writeline(self, string: str):
        def _to_bytes(s: str):
            return bytes(s, "utf-8")

        if (stdin := self.stdin) is not None:
            stdin.write(_to_bytes(string.replace("\r", "\x16\r") + self.line_break))

    def cleanup(self):
        # print "cleaning up"
        if self.p:
            self.p = None


class TestReader:
    def __init__(self, test_file: str):
        self.line_num = 0
        with open(test_file, newline="") as f:
            self.data = f.read().split("\n")
        self.soft = False
        self.deferrable = False
        self.optional = False

    def next(self):
        self.msg = None
        self.form = None
        self.out = ""
        self.ret = None

        while self.data:
            self.line_num += 1
            line = self.data.pop(0)
            if re.match(r"^\s*$", line):  # blank line
                continue
            elif line[0:3] == ";;;":  # ignore comment
                continue
            elif line[0:2] == ";;":  # output comment
                self.msg = line[3:]
                return True
            elif line[0:5] == ";>>> ":  # settings/commands
                settings = {}
                exec(line[5:], {}, settings)
                if "soft" in settings:
                    self.soft: bool = settings["soft"]
                if "deferrable" in settings and settings["deferrable"]:
                    self.deferrable = "\nSkipping deferrable and optional tests"
                    return True
                if "optional" in settings and settings["optional"]:
                    self.optional = "\nSkipping optional tests"
                    return True
                continue
            elif line[0:1] == ";":  # unexpected comment
                raise Exception(
                    "Test data error at line %d:\n%s" % (self.line_num, line)
                )
            self.form = line  # the line is a form to send

            # Now find the output and return value
            while self.data:
                line = self.data[0]
                if line[0:3] == ";=>":
                    self.ret = line[3:]
                    self.line_num += 1
                    self.data.pop(0)
                    break
                elif line[0:2] == ";/":
                    self.out = self.out + line[2:] + sep
                    self.line_num += 1
                    self.data.pop(0)
                else:
                    self.ret = ""
                    break
            if self.ret != None:
                break

        if self.out[-1:] == sep and not self.ret:
            # If there is no return value, output should not end in
            # separator
            self.out = self.out[0:-1]
        return self.form


args = parser.parse_args(sys.argv[1:])
# Workaround argparse issue with two '--' on command line
if sys.argv.count("--") > 0:
    args.mal_cmd = sys.argv[sys.argv.index("--") + 1 :]

if args.rundir:
    os.chdir(args.rundir)

if args.log_file:
    log_file = open(args.log_file, "a")
if args.debug_file:
    debug_file = open(args.debug_file, "a")

runner = Runner(args.mal_cmd, line_break="\r\n" if args.crlf else "\n")
test_reader = TestReader(args.test_file)


def assert_prompt(runner: Runner, prompts: Iterable[str], timeout: float):
    # Wait for the initial prompt
    header = runner.read_to_prompt(prompts, timeout=timeout)
    if not header == None:
        if header:
            log("Started with:\n%s" % header)
    else:
        log("Did not receive one of following prompt(s): %s" % repr(prompts))
        log("    Got      : %s" % repr(runner.buf))
        sys.exit(1)


# Wait for the initial prompt
try:
    assert_prompt(runner, [r"[^\s()<>]+> "], args.start_timeout)
except:
    _, exc, _ = sys.exc_info()
    log("\nException: %s" % repr(exc))
    log("Output before exception:\n%s" % runner.buf)
    sys.exit(1)

# Send the pre-eval code if any
if args.pre_eval:
    sys.stdout.write("RUNNING pre-eval: %s" % args.pre_eval)
    runner.writeline(args.pre_eval)
    assert_prompt(runner, [r"[^\s()<>]+> "], args.test_timeout)

test_cnt = 0
pass_cnt = 0
fail_cnt = 0
soft_fail_cnt = 0
failures: list[str] = []


class TestTimeout(Exception):
    pass


while test_reader.next():
    if args.deferrable == False and test_reader.deferrable:
        log(test_reader.deferrable)
        break

    if args.optional == False and test_reader.optional:
        log(test_reader.optional)
        break

    if test_reader.msg != None:
        log(test_reader.msg)
        continue

    if test_reader.form == None:
        continue

    log(
        "TEST: %s -> [%s,%s]"
        % (repr(test_reader.form), repr(test_reader.out), test_reader.ret),
        end="",
    )

    # The repeated form is to get around an occasional OS X issue
    # where the form is repeated.
    # https://github.com/kanaka/mal/issues/30
    ret = test_reader.ret if test_reader.ret else ""
    expects = [
        ".*%s%s%s" % (sep, test_reader.out, re.escape(ret)),
        ".*%s.*%s%s%s" % (sep, sep, test_reader.out, re.escape(ret)),
    ]

    runner.writeline(test_reader.form)
    try:
        test_cnt += 1
        res = runner.read_to_prompt(
            [r"\r\n[^\s()<>]+> ", r"\n[^\s()<>]+> "], timeout=args.test_timeout
        )
        # print "%s,%s,%s" % (idx, repr(p.before), repr(p.after))
        if res == None:
            log(" -> TIMEOUT (line %d)" % test_reader.line_num)
            raise TestTimeout("TIMEOUT (line %d)" % test_reader.line_num)
        elif test_reader.ret == "" and test_reader.out == "":
            log(" -> SUCCESS (result ignored)")
            pass_cnt += 1
        elif re.search(expects[0], res, re.S) or re.search(expects[1], res, re.S):
            log(" -> SUCCESS")
            pass_cnt += 1
        else:
            if test_reader.soft and not args.hard:
                log(" -> SOFT FAIL (line %d):" % test_reader.line_num)
                soft_fail_cnt += 1
                fail_type = "SOFT "
            else:
                log(" -> FAIL (line %d):" % test_reader.line_num)
                fail_cnt += 1
                fail_type = ""
            log("    Expected : %s" % repr(expects[0]))
            log("    Got      : %s" % repr(res))
            failed_test = """%sFAILED TEST (line %d): %s -> [%s,%s]:
    Expected : %s
    Got      : %s""" % (
                fail_type,
                test_reader.line_num,
                test_reader.form,
                repr(test_reader.out),
                test_reader.ret,
                repr(expects[0]),
                repr(res),
            )
            failures.append(failed_test)
    except:
        _, exc, _ = sys.exc_info()
        log("\nException: %s" % repr(exc))
        log("Output before exception:\n%s" % runner.buf)
        sys.exit(1)

if len(failures) > 0:
    log("\nFAILURES:")
    for f in failures:
        log(f)

results = """
TEST RESULTS (for %s):
  %3d: soft failing tests
  %3d: failing tests
  %3d: passing tests
  %3d: total tests
""" % (
    args.test_file,
    soft_fail_cnt,
    fail_cnt,
    pass_cnt,
    test_cnt,
)
log(results)

debug("\n")  # add some separate to debug log

if fail_cnt > 0:
    sys.exit(1)
sys.exit(0)
