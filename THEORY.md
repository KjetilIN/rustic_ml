# Neural Networks Explained 

> This markdown file contains theory and notes from my learning process. 

## Table of Content 

- [Neural Networks Explained](#neural-networks-explained)
  - [Table of Content](#table-of-content)
  - [Perceptron](#perceptron)
  - [Sigmoid:  Learning and Activation functions](#sigmoid--learning-and-activation-functions)
  - [The architecture of neural networks](#the-architecture-of-neural-networks)
  - [Simple network to classify handwritten digits](#simple-network-to-classify-handwritten-digits)
  - [Gradient Decent](#gradient-decent)
  - [Feedforward](#feedforward)
    - [Notation for a specific weight](#notation-for-a-specific-weight)
    - [Notation for a bias or activation](#notation-for-a-bias-or-activation)
    - [Notation of a activation](#notation-of-a-activation)
  - [Backpropagation](#backpropagation)
    - [Two Assumptions for Backpropagation](#two-assumptions-for-backpropagation)


## Perceptron

Perceptron were developed in the 1950s by Frank Rosenblatt. Today, it is more common to use other types of neurons.

They work by taking a simple set of binary inputs.

$$ x_1, x_2, ..., x_n $$

And the output of the neuron is determined by the sum of all the weights multiplied with the binary outputs, based on if this value is less or greater than a threshold value. 

$$
    \text{output} = 
    \begin{cases} 
        0 & \text{if} \quad \sum_j w_j x_j \leq \text{threshold} \\ 
        1 & \text{if} \quad \sum_j w_j x_j > \text{threshold} 
    \end{cases}
$$

The different weights and thresholds changes the decision making of the network. Changing the weights can make a input be more valued. 

A simple perceptron is not a good model. It is too simple. The image below shows a network of neurons: 

Each column of perceptron is called a layer. The first layer is often called the input layer. And the layers in the middle are called the hidden layers. The output layer is the last layer in the network.

We can change the output notation by making it the dot product between the weights and binary input. Then the threshold can be the bias for each connection. Each bias is equal to the negative value of the threshold value.

$$
    \text{output} = 
    \begin{cases} 
        0 & \text{if} \quad w \times x + b \leq 0\\ 
        1 & \text{if} \quad w \times x + b > 0
    \end{cases}
$$

Bias is a measurement of how easy it is to get the perceptron to output a 1. When a neuron outputs a 1, it is also knowns as when a neuron fires. 

## Sigmoid:  Learning and Activation functions  

Imagine that we have a network that is going to learn to recognize hand written digits. 

First we note that: __a small change in bias and weights will lead to a small change in the output__. Using this we can make changes to the network so that it behaves as we want it to!

> For example, suppose the network was mistakenly classifying an image as an “8” when it should be a “9”. We could figure out how to make a small change in the weights and biases so the network gets a little closer to classifying the image as a “9”. And then we’d repeat this, changing the weights and biases over and over to produce better and better output. The network would be learning. <br>
>
> Micheal Nielsen


**This is not what will happen with the perceptron network!** It can instead cause the network to completely flip a value from 0 to 1, and that changes the behavior. It makes the "learning" harder to track as well. Therefore, a new type of neuron is introduced: __sigmoid neuron__. The key difference is that the output is between 0 and 1. The output is now using the sigmoid function to the output:

$$ \sigma (z) = \frac{1}{1 + e^{-z}} $$

$$ \text{output} = \sigma (wx + b) $$


Sigmoid is a activation function. There is also other activation functions: https://en.wikipedia.org/wiki/Activation_function


The change in the in output can be approximated with calculus: 

$$
\Delta \text{output} \approx \sum_j \frac{\partial \text{output}}{\partial w_j} \Delta w_j + \frac{\partial \text{output}}{\partial b} \Delta b
$$


## The architecture of neural networks 

The different layers in a network are:
- Input layer
- Hidden layer(s)
- Output layer


The design of the input and output layer is simple. For example with handwritten digit. We can make the input layer contain the same amount of pixels as the image with the neural network. And the output has to be an number between 0-9 - i.e 10 output neurons. The design of the hidden layers are quite hard. 

> While the design of the input and output layers of a neural network is often straight1 forward, there can be quite an art to the design of the hidden layers.
>
> Micheal Nielsen

There is no rule of thumb. Instead researches has a suggested a set of heuristics for designing the hidden layers. For example these heuristics can help determine the tradeoff with the amount of layers vs. time required to train the network. 

A network where the output of one layer is used as input to the next layer is called a **feedforward network**. This means that the network does not contain a loop. Some networks do include a loop, also known as **recurrent neural networks**. 


## Simple network to classify handwritten digits

We can try to solve the classification problem with a three layered network. Each digit is 28x28 pixels. Each input should contain each pixel. Therefore we can add 784 neurons in the input layer. 

Then we add 15 neurons in the second layer, and the output layer has 10 neurons. One output neuron per digit. Therefore, if a neuron fires, we know what digit it is. 

> You might wonder why we use 10 output neurons. After all, the goal of the network is to tell us which digit (0,1,2,...,9) corresponds to the input image. A seemingly natural way of doing that is to use just 4 output neurons, treating each neuron as taking on a binary value, depending on whether the neuron’s output is closer to 0 or to 1. Four neurons are enough to encode the answer, since 24 = 16 is more than the 10 possible values for the input digit. Why should our network use 10 neurons instead? Isn’t that inefficient? The ultimate justification is empirical: we can try out both network designs, and it turns out that, for this particular problem, the network with 10 output neurons learns to recognize digits better than the network with 4 output neurons. 
> 
> Micheal Nielsen

We can argue that 10 output neurons makes sense if a all the hidden neurons each fire when a given feature is present. Then it makes sense that 10 works - 4 output neurons would try to find the more significant features which will be very hard to determine. 


**NB!** Note that applying the __sigmoid__ function to a vector/matrix, it is the same as applying it to each element in the vector/matrix


## Gradient Decent 

To explain Gradient Decent, we will use the classification of written digits. Each input is a vector of the pixel values. 

What we would like is a algorithm that is able to find the weights and biases so that the output from the network approximates y(x) for all training inputs x. We can compute the cost with a **cost function**:

$$
C(w, b) \equiv \frac{1}{2n} \sum_x \| y(x) - a \|^2
$$

- W: all the weights of the network
- b: all the biases from the network
- n: total number of training inputs
- a: vector of outputs from the network when x is input and the sum is over all training inputs, x. 


The cost function, C, is called a quadratic cost function is also known as the **mean squared error**. The output from the cost function is low when the network is able to correctly classify the digits - i.e the cost function tells us how "off" our current network is for correctly classifying the network based on the training sets. 

To lower the cost function we need to find the correct set of weights and biases. This is achieved by __gradient descent__.

> Why introduce the quadratic cost? After all, aren’t we primarily interested in the number of images correctly classified by the network? Why not try to maximize that number directly, rather than minimizing a proxy measure like the quadratic cost? The problem with that is that the number of images correctly classified is not a smooth function of the weights and biases in the network. For the most part, making small changes to the weights and biases won’t cause any change at all in the number of training images classified correctly. That makes it difficult to figure out how to change the weights and biases to get improved performance. If we instead use a smooth cost function like the quadratic cost it turns out to be easy to figure out how to make small changes in the weights and biases so as to get an improvement in the cost. That’s why we focus first on minimizing the quadratic cost, and only after that will we examine the classification accuracy.
> 
> Micheal Nielsen


To minimize the cost function, we can image that it is plotted in a graph. With more training sets, the more dimensions. With only two training input, we can even visualize it in a graph. (It helps to visualize it with only to variables): 

![image](https://github.com/KjetilIN/rustic_ml/assets/66110094/b0c3ea71-385b-4474-943d-8efeffc717eb)

The global minimum for the function is what we are after. We can find the minimum by using calculus. We can derive the function, but this is very hard with more than a few variables. 

We can instead solve it by thinking about the problem differently. Imagine that the cost function is a valley, and we have a ball rolling down the slope of the valley. The ball would eventually roll to the bottom. We chose a random starting point of the ball and let it start rolling. 

This is where **Gradient** comes in. The notation of gradient can be looked as a vector. It points up the slope. Meaning the opposite direction points towards the bottom of the function. It is denoted with the upside down triangle. It tells us that this vector is in fact a gradient. Movement with using the gradient we will achieve the minimum of the cost function.  

The gradient vector is (by taking the partial derivative of all variables, as a vector): 

$$
\nabla C \equiv \left( \frac{\partial C}{\partial v_1}, \frac{\partial C}{\partial v_2}, \ldots, \frac{\partial C}{\partial v_m} \right)^T
$$

Then we can change the variables such that the change in cost function is negative:

$$
\Delta v = -\eta \nabla C
$$

The learning rate is denoted with the greek eta. It tells how fast the change is cost function. The higher it is, the faster the cost function is lowered, but it is harder/impossible to make the cost approach 0 with a high learning rate. 

**We can then use this change to change all parameters in the correct direction to lower the cost function!**


This is very computational costly! Imagine a million parameters. That would be trillion of partial derivatives. There are different versions of gradient decent that we can use. 

The idea is to find the weights and biases by taking them each as partial derivatives of the cost function. 

An idea called **stochastic gradient decent** can be used to minimize the learning rate, by estimating the gradient on a small sample of randomly chosen training inputs. By choosing a random set of __m__ training inputs, it turns out that if __m__ is large that it is about the same as the gradient function: 

$$
\nabla C \approx \frac{1}{m} \sum_{j=1}^{m} \nabla C_{X_{j}}
$$

> By averaging over this small sample it turns out that we can quickly get a good estimate of the true gradient ∇C, and this helps speed up gradient descent, and thus learning.
>
> Micheal Nielsen


It is much faster to choose a small batch and then apply this gradient to the full batch. 

## Feedforward

This is a fast matrix-based approach for computing the output of the network. By feeding the input through the network through all the layers. Here is some important notation for the network: 


### Notation for a specific weight
First notation of the weight: 

$$
W_{jk}^l
$$

The weight for the connection from the k-th neuron in the (l −1)-th layer to the j-th neuron in the l-th layer. 

![image](https://github.com/KjetilIN/rustic_ml/assets/66110094/77b4eefa-6bdc-411d-9ad8-3634f32c697b)


### Notation for a bias or activation

$$
a_{j}^l
$$

- l-th layer
- j-th neuron (counted from top down)

### Notation of a activation 

The full notation for a single activation is then: 

$$
a_j^l = \sigma \left( \sum_k w_{jk}^l a_k^{l-1} + b_j^l \right)
$$

This is simplified to: 

$$
a^l = \sigma ( w^l a^{l-1} + b^l )
$$


> This expression gives us a much more global way of thinking about how the activations in one layer relate to activations in the previous layer: we just apply the weight matrix to the activations,then add the bias vector,and finally apply the σ function
>
> Micheal Nielsen

## Backpropagation 

Backpropagation is an algorithm for computing the gradient of the cost function. It is not a simple fast algorithm but it gives insights into how changing weights and biases changes the behavior of the overall network. 

The goal is to compute these partial derivatives:

$$
\frac{\partial C}{\partial w}, \frac{\partial C}{\partial b}
$$


### Two Assumptions for Backpropagation

1. 
