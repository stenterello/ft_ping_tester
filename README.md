# ft_ping_tester

This projects aims to build a simple tester to compare behaviour and output of ping (v2.0) and 42 school project "ft_ping", focused on replicate the famous ICMP tool.

## Description
The tester is divided into four parts, each one focusing on a particular aspect:
- Error handling: exit codes, output and consistency between errors and arguments passed to the two executables;
- Output: comparing of standard output of the two executables, running different tests - involving options which will you find listed below (of course is a WIP);
- Packet compliance: this part focuses on comparing the effective packets sent with the ICMP protocol. Of course it is WIP and of course it will compare only functional elements of the packet.
- Performance: different tests focused on comparing the coherence of the two executables (i.e. number of packets sent during a flood ping).

## Usage

To use this tester (which is still WIP) you have to run `get_inetutils.sh` in the project root and set your ping clone project in config.toml. If something goes wrong, `ft_ping_tester` will panic, warning about some files not found.
To start the packets compliance tester, sudo privilege is needed - because of this, the crate is made of two executables, `ft_ping_tester` and `interceptor`: the latter is to perform packets interception and must be runned as sudo (all of this is made by `ft_ping_tester` itself).

```
git clone https://github.com/stenterello/ft_ping_tester.git
cd ft_ping_tester
bash get_inetutils.sh
cargo build --bin ft_ping_tester --features="ft_ping_tester_deps"
cargo build --bin interceptor --features="interceptor_deps"
cargo run
```

To generate docs and open them in your browser it is, as usual,
```  
cargo doc --open
```

## Notes
- To capture your ft_ping's output, is probably necessary to add this function to your code, since the buffering mode of output could stop the subprocess from sending bytes through its own pipe
    ```
    setvbuf()...
    ```
- Since the argp_parse version used inside inetutils-2.0 substitutes "\`" with "'" in error prints (in particular in `Try 'ping --help' of 'ping --usage' for more information.`), I inserted a sed command in `get_inetutils.sh` to replace the hardcoded string in question. It is not elegant nor right, but still could not find solution to this.

- To change tests cases, use `tests.json` or define your own json file following the given example's structure and write its path into `config.toml`;
- To change binary or projects path, use `config.toml`.

>_m0nt4lb4n0
