# website-counter

Do you miss the good old days of the internet, when every website had a shiny counter at the bottom to show off how many visitors they had? Do you want to bring back the nostalgia and the fun of the web 1.0 era? Do you want to impress your friends and enemies with your amazing traffic stats? If you answered yes to any of these questions, then you need Website Counter!

Website Counter is a simple and easy-to-use component that you can add to your website with just a few lines of code. It will display a beautiful and customizable counter that will update automatically with every page load. You can customize colors, fonts and sizes to ensure it won't match your 90s website design, just as counters are supposed to. 

And you can display basic statistics like total Unique Users, Sessions and Page Views!

Website Counter is free, fast, totally unsecure and compatible with any website platform. Like, it works with HTML, WordPress, Joomla, Drupal and probably many more that support HTML. It even uses and supports SSL and HTTPS, so don’t you to worry about security issues. Website Counter is the ultimate solution for your website counting needs!

## How to use Website Counter

Using Website Counter is very easy. Just follow these simple steps:

* Download the file [web/index.js](web/index.js) to your website
* use it by adding it to your page via `<script src="path_to_the_file/index.js"></script>`, and then add `<website-counter></website-counter>` to the place where you want to counter to appear
* You can really put it anywhere, but we recommand to put it someplace your website users won't expect it, like in the middle of a sentence, for maximal visibility.
* PROFIT!!!

Enjoy your website counter and watch your numbers grow!

## Going wild (aka Usage)

To make sure that you get to make it as ugly as possible, there are some parameters that will help you like a lot!

```html
<website-counter 
    show="visitors|sessions|page_views" 
    background="red|#000|linear-gradient(to bottom, #999 0%, #ccc 40%, #999 100%)"
    color="green|#fff"
    font="Comic Sans MS, Comic Sans|Times New Roman|your own bad idea"
    size="2rem|500px"
    >
    Add javascript or stop visiting my website!
</website-counter>
```

As you can see, there are plenty of ways to customize this! Let's take a look into the details:

### show="visitors|sessions|page_views"

show has the possible values `visitors`, `sessions`, and `page_views`. The first number gives you a rough estimate of the number of unique visitors to your website, the second tells you how many times those visitors came to your website, and page_views tell you the total number of visitors.

You can obviously add all three by, for instance like this

```html
    <h2>Visitors</h2>
    <website-counter></website-counter>
    <h2>Visits</h2>
    <website-counter show="sessions" background="red"></website-counter>
    <h2>Page Views</h2>
    <website-counter show="page_views" font="Comic Sans MS, Comic Sans"></website-counter>
```

### other arguments

`background`, `color`, `font`, and `size` are for styling this thing. They are using their respective css values. If you don't know what I'm saying, just leave them alone.

## Features

Website Counter has many features that make it the best counter component for your website. Here are some of them:

* 100% free and no registration required
* No ads or spam
* Fast and totally unreliable
* Secure and SSL-compatible
* Customizable and absolutely not stylish
* Compatible with any website platform, because, well, it's HTML and javascript
* Zero-dependencies
* Adds basic statistics and analytics for everyone to see!
* Neither responsive and mobile-friendly!


## Why use Website Counter

It will motivate you and your visitors to keep coming back to your website and refresh the page to see the counter increase! It's endless and free fun! 

## FAQ

Here are some frequently asked questions about Website Counter:

* Q: Is Website Counter really free?
* A: Yes, Website Counter is 100% free and there are no hidden fees or charges.
---
* Q: How accurate is Website Counter?
* A: Website Counter is not very accurate, even though it uses advanced algorithms like "Sum" to count your visitors. Neither bots nor spam is filtered. Why would it? It increases the number, boya!
---
* Q: How many counters can I use on my website?
* A: You can use as many counters as you want on your website, and we recommend to do this for an optimal impact on performance and readability.
---
* Q: Can I change the design of my counter after I install it?
* A: Yes, you can change the design of your counter anytime you want. Just read the readme, which is this document, like again. Then you should know how.
---
* Q: Can I reset or delete my counter?
* A: NO! Because: why would you?? Gah, alright. If you absolutely want to... Open an issue on github and i'll look into it.
---
* Q: Any guarantees that my data will be backupped and such?
* A: Absolutely not. The data is running on a [render free tier](https://render.com) and probably gone tomorrow.
---
* Q: User privacy is very important for me. What do you do with the data?
* A: Well, the counter is public. The data to track the Unique User is randomized for each website, and not guaranteed to be unique between multiple websites, so it's impossible for me to track users over multiple pages that are including the counter. The data is stored on a [render free tier](https://render.com)


## Support

This thing is offered like with no support or guarantees. Use it at your own risk.