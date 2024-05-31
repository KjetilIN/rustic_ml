#[cfg(test)]
mod tests {
    use rustic_ml::nn::activation::*;

    #[test]
    fn test_activation_step(){
        assert_eq!(step(-3.4), 0.0);
        assert_eq!(step(3.4), 1.0);
        assert_eq!(step(0.0), 1.0)
    }

    #[test]
    fn test_activation_sigmoid(){
        assert_eq!(sigmoid(0.0), 0.5);
        assert_eq!(sigmoid(-6.0), 0.0024726237);
        assert_eq!(sigmoid(6.0), 0.9975274);
    }

    #[test]
    fn test_activation_relu(){
        assert_eq!(relu(0.0), 0.0);
        assert_eq!(relu(1.12), 1.12);
        assert_eq!(relu(-0.123), 0.0);

    }
    #[test]
    fn test_activation_leaky_relu(){
        assert_eq!(leaky_relu(0.0), 0.0);
        assert_eq!(leaky_relu(1.12), 1.12);
        assert_eq!(leaky_relu(-100.0), -10.0);
    }

    #[test]
    fn test_activation_softplus(){
        assert_eq!(softplus(-6.0), 0.002475652);
        assert_eq!(softplus(0.0), 0.6931472);
        assert_eq!(softplus(1.0), 1.3132616);
        assert_eq!(softplus(1.0), 1.3132616);
        assert_eq!(softplus(6.0), 6.0024753);
    }

    #[test]
    fn test_activation_elu(){
        assert_eq!(elu(6.0, 1.0), 6.0);
        assert_eq!(elu(1.0, 1.0), 1.0);
        assert_eq!(elu(0.0, 1.0), 0.0);
        assert_eq!(elu(-6.0, 5.0), -4.987606);
        assert_eq!(elu(-1.0, 5.0), -3.1606028);
    }
}