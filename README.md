# ft_ping_tester

This projects aims to build a simple tester to compare behaviour and output of ping (v2.0) and 42 school project "ft_ping", focused on replicate the famous ICMP tool.

## Description
The tester is divided into three parts, each one focusing on a particular aspect:
- Error handling: exit codes, output and consistency between errors and arguments passed to the two executables;
- Output: comparing of standard output of the two executables, running different tests - involving options which will you find listed below (of course is a WIP);
- Performance: different tests focused on comparing the coherence of the two executables (i.e. number of packets sent during a flood ping).

## Usage

## Notes
- To capture your ft_ping's output, is probably necessary to add this function to your code, since the buffering mode of output could stop the subprocess from sending bytes through its own pipe
    ```
    setvbuf()...
    ```
- Actually supported options:
    -

- To change tests cases, use tests.json or define your own json file following the given example's structure and write its path into conf.toml;
- To change binary or projects path, use conf.toml (be aware that, actually, build.rs will still try to download and compile inetutils version of ping and putting it in project root directory).


>_m0nt4lb4n0
