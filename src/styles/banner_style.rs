pub const CSS: &str = r#"

@keyframes slideIn {
    from {
        transform: translateY(-30px);
        opacity: 0;
        filter: blur(10px);
    }
    to {
        transform: translateY(0);
        opacity: 1;
        filter: blur(0px);
    }
}

.title,
.topic,
.des,
.detail-btn {
    opacity: 0;
    animation: slideIn 0.5s ease-in-out forwards;
}

.title {
    animation-delay: 1s;
}

.topic {
    animation-delay: 1.2s;
}

.des {
    animation-delay: 1.4s;
}

.detail-btn {
    transition: 0.2s ease-out;
}

.detail-btn:nth-child(1) {
    animation-delay: 1.8s;
     transition: 0.5s ease-out;
}

.detail-btn:nth-child(2) {
    animation-delay: 2s;
}

"#;
