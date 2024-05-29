<a name="readme-top"></a>



<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/kingwingfly/gpt">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">gpt_ui</h3>

  <p align="center">
    A CLI and TUI for chatGPT built with Rust. And offer a core crate for chatGPT app.
    <br />
    <a href="https://github.com/kingwingfly/gpt"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/kingwingfly/gpt">View Demo</a>
    ·
    <a href="https://github.com/kingwingfly/gpt/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    ·
    <a href="https://github.com/kingwingfly/gpt/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://github.com/kingwingfly/gpt)


<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Rust][Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Download in [release page](https://github.com/kingwingfly/gpt/releases) or compile it yourself.

For windows users, there's bug upstream when handling ctrl-c, you can quit by entering empty line or select quit option.

### Prerequisites

* Install Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Compilation

1. Clone the repo
   ```sh
   git clone https://github.com/kingwingfly/gpt.git
   ```
2. Compilation
   ```sh
   cargo build --bin gpt_cli --features cli --release
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```sh
# start chatting
gpt_cli
# config
gpt_cli config
# quit
ESC
```

_For more examples, please refer to the [Documentation](https://docs.rs/gpt_core)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Feature

See the [open issues](https://github.com/kingwingfly/gpt/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Louis - 836250617@qq.com

Project Link: [https://github.com/kingwingfly/gpt](https://github.com/kingwingfly/gpt)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [chatGPT stream rust demo](https://github.com/a-poor/openai-stream-rust-demo)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/kingwingfly/gpt.svg?style=for-the-badge
[contributors-url]: https://github.com/kingwingfly/gpt/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/kingwingfly/gpt.svg?style=for-the-badge
[forks-url]: https://github.com/kingwingfly/gpt/network/members
[stars-shield]: https://img.shields.io/github/stars/kingwingfly/gpt.svg?style=for-the-badge
[stars-url]: https://github.com/kingwingfly/gpt/stargazers
[issues-shield]: https://img.shields.io/github/issues/kingwingfly/gpt.svg?style=for-the-badge
[issues-url]: https://github.com/kingwingfly/gpt/issues
[license-shield]: https://img.shields.io/github/license/kingwingfly/gpt.svg?style=for-the-badge
[license-url]: https://github.com/kingwingfly/gpt/blob/master/LICENSE.txt
[product-screenshot]: images/screenshot.png
[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=orange
[Rust-url]: https://www.rust-lang.org
